#pragma warning disable CS8981
using System.Runtime.InteropServices;

namespace CuteLights.Sdk {

    public class LightDiscoverer {
        public static List<Light> Discover() {
            unsafe {
                var ld = NativeMethods.light_discoverer_new();
                var lights = new List<Light>();
                var l = NativeMethods.light_discoverer_next(ld);
                while (l != null) {
                    lights.Add(new Light(l));
                    l = NativeMethods.light_discoverer_next(ld);
                }
                NativeMethods.light_discoverer_free(ld);
                return lights;
            }
        }
    }

    public class Light {
        internal unsafe light* _light;

        public Name { get; private set; }
        public Id { get; private set; }

        internal unsafe Light(light* l) {
            _light = l;
            // These are string func's that dont change so we can cache them and 
            // not have to cross the FFI boundary every time and decode the string
            unsafe {
                Name = Marshal.PtrToStringAnsi((IntPtr)NativeMethods.light_get_name(_light)) ?? "";
                Id = Marshal.PtrToStringAnsi((IntPtr)NativeMethods.light_get_id(_light)) ?? "";
            }
        }

        ~Light() {
            unsafe {
                NativeMethods.light_free(_light);
            }
        }

        public Task<bool> SetOn(bool on) {
            return Task.Run(() => {
                unsafe {
                    return NativeMethods.light_set_on(_light, on);
                }
            });
        }

        public Task<bool> SetColor(long h, long s, long b) {
            return Task.Run(() => {
                unsafe {
                    return NativeMethods.light_set_color(_light, h, s, b);
                }
            });
        }

        public Task<bool> SetBrightness(long brightness) {
            return Task.Run(() => {
                unsafe {
                    return NativeMethods.light_set_brightness(_light, brightness);
                }
            });
        }

        public long Brightness {
            get {
                unsafe {
                    return NativeMethods.light_get_brightness(_light);
                }
            }
        }

        public long Hue {
            get {
                unsafe {
                    return NativeMethods.light_get_hue(_light);
                }
            }
        }

        public long Saturation {
            get {
                unsafe {
                    return NativeMethods.light_get_saturation(_light);
                }
            }
        }

        public bool IsOn {
            get {
                unsafe {
                    return NativeMethods.light_get_is_on(_light);
                }
            }
        }   

        public bool SupportsColor {
            get {
                unsafe {
                    return NativeMethods.light_get_supports_color(_light);
                }
            }
        }
    }

    public class Frame {
        private unsafe frame* _frame;

        public Frame() {
            unsafe {
                _frame = NativeMethods.frame_new();
            }
        }

        ~Frame() {
            unsafe {
                NativeMethods.frame_free(_frame);
            }
        }

        public void Clear() {
            unsafe {
                NativeMethods.frame_clear(_frame);
            }
        }

        public void SetOn(Light l, bool on) {
            unsafe {
                NativeMethods.frame_set_on(_frame, l._light, on);
            }
        }

        public void SetColor(Light l, long h, long s, long b) {
            unsafe {
                NativeMethods.frame_set_color(_frame, l._light, h, s, b);
            }
        }

        public void SetBrightness(Light l, long brightness) {
            unsafe {
                NativeMethods.frame_set_brightness(_frame, l._light, brightness);
            }
        }

        public void Run() {
            unsafe {
                NativeMethods.frame_run(_frame);
            }
        }
    }

   

    internal static unsafe partial class NativeMethods {
        const string __DllName = "libcutelights.so";

        [LibraryImport(__DllName, EntryPoint = "light_set_on")]
        [UnmanagedCallConv(CallConvs = new Type[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
        [return: MarshalAs(UnmanagedType.U1)]
        internal static partial bool light_set_on(light* l, [MarshalAs(UnmanagedType.U1)] bool on);

        [LibraryImport(__DllName, EntryPoint = "light_set_color")]
        [UnmanagedCallConv(CallConvs = new Type[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
        [return: MarshalAs(UnmanagedType.U1)]
        internal static partial bool light_set_color(light* l, long h, long s, long b);

        [LibraryImport(__DllName, EntryPoint = "light_set_brightness")]
        [UnmanagedCallConv(CallConvs = new Type[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
        [return: MarshalAs(UnmanagedType.U1)]
        internal static partial bool light_set_brightness(light* l, long brightness);

        [LibraryImport(__DllName, EntryPoint = "light_get_brightness")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial long light_get_brightness(light* l);

        [LibraryImport(__DllName, EntryPoint = "light_get_hue")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial long light_get_hue(light* l);

        [LibraryImport(__DllName, EntryPoint = "light_get_saturation")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial long light_get_saturation(light* l);

        [LibraryImport(__DllName, EntryPoint = "light_get_is_on")]
        [UnmanagedCallConv(CallConvs = new Type[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
        [return: MarshalAs(UnmanagedType.U1)]
        internal static partial bool light_get_is_on(light* l);

        [LibraryImport(__DllName, EntryPoint = "light_get_name")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial byte* light_get_name(light* l);

        [LibraryImport(__DllName, EntryPoint = "light_get_id")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial byte* light_get_id(light* l);

        [LibraryImport(__DllName, EntryPoint = "light_get_supports_color")]
        [UnmanagedCallConv(CallConvs = new Type[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
        [return: MarshalAs(UnmanagedType.U1)]
        internal static partial bool light_get_supports_color(light* l);

        [LibraryImport(__DllName, EntryPoint = "light_free")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial void light_free(light* l);

        [LibraryImport(__DllName, EntryPoint = "light_discoverer_new")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial light_discoverer* light_discoverer_new();

        [LibraryImport(__DllName, EntryPoint = "light_discoverer_next")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial light* light_discoverer_next(light_discoverer* ld);

        [LibraryImport(__DllName, EntryPoint = "light_discoverer_free")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial void light_discoverer_free(light_discoverer* ld);

        [LibraryImport(__DllName, EntryPoint = "frame_new")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial frame* frame_new();

        [LibraryImport(__DllName, EntryPoint = "frame_clear")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial void frame_clear(frame* f);

        [LibraryImport(__DllName, EntryPoint = "frame_free")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial void frame_free(frame* f);

        [LibraryImport(__DllName, EntryPoint = "frame_set_on")]
        [UnmanagedCallConv(CallConvs = new Type[] { typeof(System.Runtime.CompilerServices.CallConvCdecl) })]
        internal static partial void frame_set_on(frame* f, light* l, [MarshalAs(UnmanagedType.U1)] bool on);

        [LibraryImport(__DllName, EntryPoint = "frame_set_color")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial void frame_set_color(frame* f, light* l, long h, long s, long b);

        [LibraryImport(__DllName, EntryPoint = "frame_set_brightness")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial void frame_set_brightness(frame* f, light* l, long brightness);

        [LibraryImport(__DllName, EntryPoint = "frame_run")]
        [UnmanagedCallConv(CallConvs = [typeof(System.Runtime.CompilerServices.CallConvCdecl)])]
        internal static partial void frame_run(frame* f);


    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct light {
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct light_discoverer {
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct frame {
    }
}
