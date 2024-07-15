rm -rf bin/Release/CuteLight.Sdk.*.nupkg
dotnet pack -c Release
dotnet nuget push bin/Release/CuteLight.Sdk.*.nupkg --source https://api.nuget.org/v3/index.json --api-key $NUGET_API_KEY