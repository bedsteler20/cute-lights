
namespace CuteLights.Sdk;

public static class LightDiscoverer {
    public static Light[] Discover() {
        unsafe {
            var lights = new List<Light>();
            var ld = NativeMethods.light_discoverer_new();
            LightPtr* l;
            while ((l = NativeMethods.light_discoverer_next(ld)) != null) {
                lights.Add(new Light(l));
            }
            NativeMethods.light_discoverer_free(ld);
            return lights.ToArray();
        }
    }
}