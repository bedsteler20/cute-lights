// See https://aka.ms/new-console-template for more information

using CuteLights.Sdk;

var lights = LightDiscoverer.Discover();

Console.WriteLine("Discovered lights:");
foreach (var light in lights) {
    Console.WriteLine($"  {light.Name} ({light.Id})");
}
// var frame = new Frame();

var frame = new Frame();
var on = true;
while (true) {
    frame.SetOnAll(lights, on);
    on = !on;
    await frame.Run();
    frame.Clear();
    await Task.Delay(1000);
}
