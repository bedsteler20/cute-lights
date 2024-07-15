using System.Runtime.InteropServices;

namespace CuteLights.Sdk;

/// <summary>
/// Represents a light device.
/// </summary>
public class Light {
    internal unsafe LightPtr* ptr;

    internal unsafe Light(LightPtr* ptr) {
        this.ptr = ptr;
        Id = Marshal.PtrToStringAnsi((IntPtr)NativeMethods.light_get_id(ptr)) ?? "";
        Name = Marshal.PtrToStringAnsi((IntPtr)NativeMethods.light_get_name(ptr)) ?? "";
    }

    /// <summary>
    /// Gets the ID of the light.
    /// </summary>
    public string Id { get; private set; }

    /// <summary>
    /// Gets the name of the light.
    /// </summary>
    public string Name { get; private set; }

    public LightColor Color {
        get {
            unsafe {
                return new LightColor(NativeMethods.light_get_red(ptr), NativeMethods.light_get_green(ptr), NativeMethods.light_get_blue(ptr));
            }
        }
    }

    /// <summary>
    /// Gets the brightness level of the light.
    /// </summary>
    public int Brightness {
        get {
            unsafe {
                return NativeMethods.light_get_brightness(ptr);
            }
        }
    }

    /// <summary>
    /// Gets the red component of the light's color.
    /// </summary>
    public int Red {
        get {
            unsafe {
                return NativeMethods.light_get_red(ptr);
            }
        }
    }

    /// <summary>
    /// Gets the green component of the light's color.
    /// </summary>
    public int Green {
        get {
            unsafe {
                return NativeMethods.light_get_green(ptr);
            }
        }
    }

    /// <summary>
    /// Gets the blue component of the light's color.
    /// </summary>
    public int Blue {
        get {
            unsafe {
                return NativeMethods.light_get_blue(ptr);
            }
        }
    }

    /// <summary>
    /// Gets a value indicating whether the light is turned on.
    /// </summary>
    public bool IsOn {
        get {
            unsafe {
                return NativeMethods.light_get_is_on(ptr);
            }
        }
    }

    /// <summary>
    /// Gets a value indicating whether the light supports color.
    /// </summary>
    public bool SupportsColor {
        get {
            unsafe {
                return NativeMethods.light_get_supports_color(ptr);
            }
        }
    }

    /// <summary>
    /// Disposes the light and releases any resources associated with it.
    /// </summary>
    public void Dispose() {
        unsafe {
            NativeMethods.light_free(ptr);
        }
    }

    /// <summary>
    /// Sets the light on or off asynchronously.
    /// </summary>
    /// <param name="on">A boolean value indicating whether to turn the light on or off.</param>
    /// <returns>A task that represents the asynchronous operation. The task result contains a boolean value indicating whether the operation was successful.</returns>
    public Task<bool> SetOn(bool on) {
        return Task.Run(() => {
            unsafe {
                return NativeMethods.light_set_on(ptr, on);
            }
        });
    }

    /// <summary>
    /// Sets the color of the light asynchronously.
    /// </summary>
    /// <param name="red">The red component of the color (0-255).</param>
    /// <param name="green">The green component of the color (0-255).</param>
    /// <param name="blue">The blue component of the color (0-255).</param>
    /// <returns>A task that represents the asynchronous operation. The task result contains a boolean value indicating whether the operation was successful.</returns>
    /// <exception cref="NotSupportedException">Thrown when the light does not support color.</exception>
    /// <exception cref="ArgumentOutOfRangeException">Thrown when the red, green, or blue value is outside the valid range (0-255).</exception>
    public Task<bool> SetColor(int red, int green, int blue) {
        if (!SupportsColor) {
            throw new NotSupportedException("Light does not support color");
        }
        if (red < 0 || red > 255) {
            throw new ArgumentOutOfRangeException(nameof(red), "Red must be between 0 and 255");
        }
        if (green < 0 || green > 255) {
            throw new ArgumentOutOfRangeException(nameof(green), "Green must be between 0 and 255");
        }
        if (blue < 0 || blue > 255) {
            throw new ArgumentOutOfRangeException(nameof(blue), "Blue must be between 0 and 255");
        }
        return Task.Run(() => {
            unsafe {
                return NativeMethods.light_set_color(ptr, (byte)red, (byte)green, (byte)blue);
            }
        });
    }

    /// <summary>
    /// Sets the color of the light.
    /// </summary>
    /// <param name="color">The color to set.</param>
    /// <returns>A task that represents the asynchronous operation. The task result contains a boolean value indicating whether the color was set successfully.</returns>
    public Task<bool> SetColor(LightColor color) {
        return SetColor(color.Red, color.Green, color.Blue);
    }

    /// <summary>
    /// Sets the brightness level of the light asynchronously.
    /// </summary>
    /// <param name="brightness">The brightness level (0-100).</param>
    /// <returns>A task that represents the asynchronous operation. The task result contains a boolean value indicating whether the operation was successful.</returns>
    /// <exception cref="ArgumentOutOfRangeException">Thrown when the brightness value is outside the valid range (0-100).</exception>
    public Task<bool> SetBrightness(int brightness) {
        if (brightness < 0 || brightness > 100) {
            throw new ArgumentOutOfRangeException(nameof(brightness), "Brightness must be between 0 and 100");
        }
        return Task.Run(() => {
            unsafe {
                return NativeMethods.light_set_brightness(ptr, (byte)brightness);
            }
        });
    }

    public override string ToString() {
        return $"{Name} ({Id})";
    }

    ~Light() {
        Dispose();
    }
}
