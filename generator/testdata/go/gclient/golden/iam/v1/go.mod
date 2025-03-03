module github.com/google-cloud-rust/generator/testdata/go/gclient/golden/iam/v1

go 1.23.2

replace github.com/google-cloud-rust/generator/testdata/go/gclient/golden/typez => ../../typez

replace github.com/google-cloud-rust/generator/testdata/go/gclient/golden/wkt => ../../wkt

require (
	cloud.google.com/go/auth v0.12.0
	github.com/google-cloud-rust/generator/testdata/go/gclient/golden/typez v0.0.0-00010101000000-000000000000
	github.com/google-cloud-rust/generator/testdata/go/gclient/golden/wkt v0.0.0-00010101000000-000000000000
)

require (
	cloud.google.com/go/compute/metadata v0.5.1 // indirect
	golang.org/x/sys v0.25.0 // indirect
	google.golang.org/protobuf v1.35.2 // indirect
)
