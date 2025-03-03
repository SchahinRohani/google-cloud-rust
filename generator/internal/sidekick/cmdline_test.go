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
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestParseArgs(t *testing.T) {
	args := []string{
		"-project-root", "../..",
		"-specification-format", "openapi",
		"-specification-source", specificationSource,
		"-service-config", secretManagerServiceConfig,
		"-source-option", fmt.Sprintf("googleapis-root=%s", googleapisRoot),
		"-language", "rust",
		"-output", outputDir,
		"-template-dir", "generator/templates",
		"-codec-option", "copyright-year=2024",
		"-codec-option", "package-name-override=secretmanager-golden-openapi",
		"-codec-option", "package:wkt=package=gcp-sdk-wkt,path=src/wkt,source=google.protobuf",
		"-codec-option", "package:gax=package=gcp-sdk-gax,path=src/gax,feature=sdk_client",
		"-codec-option", "package:google-cloud-auth=package=google-cloud-auth,path=auth",
		"generate",
	}
	got, err := parseArgsExplicit(args)
	if err != nil {
		t.Fatal(err)
	}
	want := &CommandLine{
		Command:             "generate",
		ProjectRoot:         "../..",
		SpecificationFormat: "openapi",
		SpecificationSource: specificationSource,
		ServiceConfig:       secretManagerServiceConfig,
		Source: map[string]string{
			"googleapis-root": googleapisRoot,
		},
		Language:    "rust",
		Output:      outputDir,
		TemplateDir: templateDir,
		Codec: map[string]string{
			"copyright-year":            "2024",
			"package-name-override":     "secretmanager-golden-openapi",
			"package:wkt":               "package=gcp-sdk-wkt,path=src/wkt,source=google.protobuf",
			"package:gax":               "package=gcp-sdk-gax,path=src/gax,feature=sdk_client",
			"package:google-cloud-auth": "package=google-cloud-auth,path=auth",
		},
	}
	if diff := cmp.Diff(want, got); diff != "" {
		t.Errorf("mismatched merged config (-want, +got):\n%s", diff)
	}
}

func TestDefaults(t *testing.T) {
	root := t.TempDir()
	args := []string{
		"-project-root", root,
		"generate",
	}
	got, err := parseArgsExplicit(args)
	if err != nil {
		t.Fatal(err)
	}
	want := &CommandLine{
		Command:     "generate",
		ProjectRoot: root,
		Source:      map[string]string{},
		Codec:       map[string]string{},
	}
	if diff := cmp.Diff(want, got); diff != "" {
		t.Errorf("mismatched merged config (-want, +got):\n%s", diff)
	}
}
