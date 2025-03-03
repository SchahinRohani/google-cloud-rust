// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package sidekick

import (
	"fmt"
	"strings"
	"time"

	"github.com/googleapis/google-cloud-rust/generator/internal/api"
	"github.com/googleapis/google-cloud-rust/generator/internal/language"
	"github.com/iancoleman/strcase"
)

type TemplateData struct {
	TemplateDir       string
	Name              string
	Title             string
	Description       string
	PackageName       string
	RequiredPackages  []string
	HasServices       bool
	CopyrightYear     string
	BoilerPlate       []string
	Imports           []string
	DefaultHost       string
	Services          []*Service
	Messages          []*Message
	NameToLower       string
	NotForPublication bool
}

type Service struct {
	Methods             []*Method
	NameToSnake         string
	NameToPascal        string
	ServiceNameToPascal string
	NameToCamel         string
	ServiceName         string
	DocLines            []string
	DefaultHost         string
}

type Message struct {
	Fields             []*Field
	BasicFields        []*Field
	ExplicitOneOfs     []*OneOf
	NestedMessages     []*Message
	Enums              []*Enum
	MessageAttributes  []string
	Name               string
	QualifiedName      string
	NameSnakeCase      string
	HasNestedTypes     bool
	DocLines           []string
	IsMap              bool
	IsPageableResponse bool
}

type Method struct {
	NameToSnake       string
	NameToCamel       string
	NameToPascal      string
	DocLines          []string
	InputTypeName     string
	OutputTypeName    string
	HTTPMethod        string
	HTTPMethodToLower string
	HTTPPathFmt       string
	HTTPPathArgs      []string
	QueryParams       []*Field
	HasBody           bool
	BodyAccessor      string
	IsPageable        bool
}

type OneOf struct {
	NameToPascal          string
	NameToSnake           string
	NameToSnakeNoMangling string
	FieldType             string
	DocLines              []string
	Fields                []*Field
}

type Field struct {
	NameToSnake           string
	NameToSnakeNoMangling string
	NameToCamel           string
	NameToPascal          string
	DocLines              []string
	FieldAttributes       []string
	FieldType             string
	JSONName              string
	AsQueryParameter      string
}

type Enum struct {
	Name          string
	NameSnakeCase string
	DocLines      []string
	Values        []*EnumValue
}

type EnumValue struct {
	DocLines []string
	Name     string
	Number   int32
	EnumType string
}

// newTemplateData creates a struct used as input for Mustache templates.
// Fields and methods defined in this struct directly correspond to Mustache
// tags. For example, the Mustache tag {{#Services}} uses the
// [Template.Services] field.
func newTemplateData(model *api.API, c language.Codec) *TemplateData {
	c.LoadWellKnownTypes(model.State)
	year, _, _ := time.Now().Date()
	return &TemplateData{
		TemplateDir:      c.TemplateDir(),
		Name:             model.Name,
		Title:            model.Title,
		Description:      model.Description,
		PackageName:      c.PackageName(model),
		RequiredPackages: c.RequiredPackages(),
		HasServices:      len(model.Services) > 0,
		CopyrightYear:    fmt.Sprintf("%04d", year),
		BoilerPlate: append(licenseHeaderBulk(),
			"",
			" Code generated by sidekick. DO NOT EDIT."),
		Imports: c.Imports(),
		DefaultHost: func() string {
			if len(model.Services) > 0 {
				return model.Services[0].DefaultHost
			}
			return ""
		}(),
		Services: mapSlice(model.Services, func(s *api.Service) *Service {
			return newService(s, c, model.State)
		}),
		Messages: mapSlice(model.Messages, func(m *api.Message) *Message {
			return newMessage(m, c, model.State)
		}),
		NameToLower:       strings.ToLower(model.Name),
		NotForPublication: c.NotForPublication(),
	}
}

func newService(s *api.Service, c language.Codec, state *api.APIState) *Service {
	return &Service{
		Methods: mapSlice(s.Methods, func(m *api.Method) *Method {
			return newMethod(m, c, state)
		}),
		NameToSnake:         c.ToSnake(s.Name),
		NameToPascal:        c.ToPascal(s.Name),
		ServiceNameToPascal: c.ToPascal(s.Name), // Alias for clarity
		NameToCamel:         c.ToCamel(s.Name),
		ServiceName:         s.Name,
		DocLines:            c.FormatDocComments(s.Documentation),
		DefaultHost:         s.DefaultHost,
	}
}

func newMessage(m *api.Message, c language.Codec, state *api.APIState) *Message {
	return &Message{
		Fields: mapSlice(m.Fields, func(s *api.Field) *Field {
			return newField(s, c, state)
		}),
		BasicFields: func() []*Field {
			filtered := filterSlice(m.Fields, func(s *api.Field) bool {
				return !s.IsOneOf
			})
			return mapSlice(filtered, func(s *api.Field) *Field {
				return newField(s, c, state)
			})
		}(),
		ExplicitOneOfs: mapSlice(m.OneOfs, func(s *api.OneOf) *OneOf {
			return newOneOf(s, c, state)
		}),
		NestedMessages: mapSlice(m.Messages, func(s *api.Message) *Message {
			return newMessage(s, c, state)
		}),
		Enums: mapSlice(m.Enums, func(s *api.Enum) *Enum {
			return newEnum(s, c, state)
		}),
		MessageAttributes: c.MessageAttributes(m, state),
		Name:              c.MessageName(m, state),
		QualifiedName:     c.FQMessageName(m, state),
		NameSnakeCase:     c.ToSnake(m.Name),
		HasNestedTypes: func() bool {
			if len(m.Enums) > 0 || len(m.OneOfs) > 0 {
				return true
			}
			for _, child := range m.Messages {
				if !child.IsMap {
					return true
				}
			}
			return false
		}(),
		DocLines:           c.FormatDocComments(m.Documentation),
		IsMap:              m.IsMap,
		IsPageableResponse: m.IsPageableResponse,
	}
}

func newMethod(m *api.Method, c language.Codec, state *api.APIState) *Method {
	return &Method{
		BodyAccessor:      c.BodyAccessor(m, state),
		DocLines:          c.FormatDocComments(m.Documentation),
		HTTPMethod:        m.PathInfo.Verb,
		HTTPMethodToLower: strings.ToLower(m.PathInfo.Verb),
		HTTPPathArgs:      c.HTTPPathArgs(m.PathInfo, state),
		HTTPPathFmt:       c.HTTPPathFmt(m.PathInfo, state),
		HasBody:           m.PathInfo.BodyFieldPath != "",
		InputTypeName:     c.MethodInOutTypeName(m.InputTypeID, state),
		NameToCamel:       strcase.ToCamel(m.Name),
		NameToPascal:      c.ToPascal(m.Name),
		NameToSnake:       strcase.ToSnake(m.Name),
		OutputTypeName:    c.MethodInOutTypeName(m.OutputTypeID, state),
		QueryParams: mapSlice(c.QueryParams(m, state), func(s *api.Field) *Field {
			return newField(s, c, state)
		}),
		IsPageable: m.IsPageable,
	}
}

func newOneOf(oneOf *api.OneOf, c language.Codec, state *api.APIState) *OneOf {
	return &OneOf{
		NameToPascal:          c.ToPascal(oneOf.Name),
		NameToSnake:           c.ToSnake(oneOf.Name),
		NameToSnakeNoMangling: c.ToSnakeNoMangling(oneOf.Name),
		FieldType:             c.OneOfType(oneOf, state),
		DocLines:              c.FormatDocComments(oneOf.Documentation),
		Fields: mapSlice(oneOf.Fields, func(field *api.Field) *Field {
			return newField(field, c, state)
		}),
	}
}

func newField(field *api.Field, c language.Codec, state *api.APIState) *Field {
	return &Field{
		NameToSnake:           c.ToSnake(field.Name),
		NameToSnakeNoMangling: c.ToSnakeNoMangling(field.Name),
		NameToCamel:           c.ToCamel(field.Name),
		NameToPascal:          c.ToPascal(field.Name),
		DocLines:              c.FormatDocComments(field.Documentation),
		FieldAttributes:       c.FieldAttributes(field, state),
		FieldType:             c.FieldType(field, state),
		JSONName:              field.JSONName,
		AsQueryParameter:      c.AsQueryParameter(field, state),
	}
}

func newEnum(e *api.Enum, c language.Codec, state *api.APIState) *Enum {
	return &Enum{
		Name:          c.EnumName(e, state),
		NameSnakeCase: c.ToSnake(c.EnumName(e, state)),
		DocLines:      c.FormatDocComments(e.Documentation),
		Values: mapSlice(e.Values, func(s *api.EnumValue) *EnumValue {
			return newEnumValue(s, e, c, state)
		}),
	}
}

func newEnumValue(ev *api.EnumValue, e *api.Enum, c language.Codec, state *api.APIState) *EnumValue {
	return &EnumValue{
		DocLines: c.FormatDocComments(ev.Documentation),
		Name:     c.EnumValueName(ev, state),
		Number:   ev.Number,
		EnumType: c.EnumName(e, state),
	}
}

func filterSlice[T any](slice []T, predicate func(T) bool) []T {
	result := make([]T, 0)
	for _, v := range slice {
		if predicate(v) {
			result = append(result, v)
		}
	}
	return result
}
func mapSlice[T, R any](s []T, f func(T) R) []R {
	r := make([]R, len(s))
	for i, v := range s {
		r[i] = f(v)
	}
	return r
}
