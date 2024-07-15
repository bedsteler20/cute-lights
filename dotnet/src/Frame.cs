namespace CuteLights.Sdk;

/// <summary>
/// Allows for running multiple commands at once on a set of lights.
/// </summary>
public class Frame {
    internal unsafe FramePtr* ptr;

    public Frame() {
        unsafe {
            ptr = NativeMethods.frame_new();
        }
    }

    /// <summary>
    /// Sets the state of a light in the frame.
    /// </summary>
    /// <param name="light">The light to set the state for.</param>
    /// <param name="on">The state to set for the light (true for on, false for off).</param>
    public void SetOn(Light light, bool on) {
        unsafe {
            NativeMethods.frame_set_on(ptr, light.ptr, on);
        }
    }

    /// <summary>
    /// Sets the on/off state for all the lights in the specified array.
    /// </summary>
    /// <param name="lights">The array of lights.</param>
    /// <param name="on">The on/off state to set.</param>
    public void SetOnAll(Light[] lights, bool on) {
        unsafe {
            foreach (var light in lights) {
                NativeMethods.frame_set_on(ptr, light.ptr, on);
            }
        }
    }

    /// <summary>
    /// Sets the color of a light in the frame.
    /// </summary>
    /// <param name="light">The light to set the color for.</param>
    /// <param name="red">The red component of the color (between 0 and 255).</param>
    /// <param name="green">The green component of the color (between 0 and 255).</param>
    /// <param name="blue">The blue component of the color (between 0 and 255).</param>
    /// <exception cref="NotSupportedException">Thrown when the light does not support color.</exception>
    /// <exception cref="ArgumentOutOfRangeException">Thrown when any of the color components are outside the valid range of 0 to 255.</exception>
    public void SetColor(Light light, int red, int green, int blue) {
        if (!light.SupportsColor) {
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
        unsafe {
            NativeMethods.frame_set_color(ptr, light.ptr, (byte)red, (byte)green, (byte)blue);
        }
    }


    /// <summary>
    /// Sets the color of a specific light.
    /// </summary>
    /// <param name="light">The light to set the color for.</param>
    /// <param name="color">The color to set.</param>
    public void SetColor(Light light, LightColor color) {
        SetColor(light, color.Red, color.Green, color.Blue);
    }

    /// <summary>
    /// Sets the color of all the lights in the specified array.
    /// </summary>
    /// <param name="lights">The array of lights.</param>
    /// <param name="red">The red component of the color (between 0 and 255).</param>
    /// <param name="green">The green component of the color (between 0 and 255).</param>
    /// <param name="blue">The blue component of the color (between 0 and 255).</param>
    /// <exception cref="ArgumentOutOfRangeException">
    /// Thrown when any of the color components are outside the valid range of 0 to 255.
    /// </exception>
    /// <exception cref="NotSupportedException">
    /// Thrown when a light in the array does not support color.
    /// </exception>
    public void SetColorAll(Light[] lights, int red, int green, int blue) {
        if (red < 0 || red > 255) {
            throw new ArgumentOutOfRangeException(nameof(red), "Red must be between 0 and 255");
        }
        if (green < 0 || green > 255) {
            throw new ArgumentOutOfRangeException(nameof(green), "Green must be between 0 and 255");
        }
        if (blue < 0 || blue > 255) {
            throw new ArgumentOutOfRangeException(nameof(blue), "Blue must be between 0 and 255");
        }
        unsafe {
            foreach (var light in lights) {
                if (!light.SupportsColor) {
                    throw new NotSupportedException("Light does not support color");
                }

                NativeMethods.frame_set_color(ptr, light.ptr, (byte)red, (byte)green, (byte)blue);
            }
        }
    }

    /// <summary>
    /// Sets the color of all lights in the specified array to the specified color.
    /// </summary>
    /// <param name="lights">The array of lights.</param>
    /// <param name="color">The color to set.</param>
    public void SetColorAll(Light[] lights, LightColor color) {
        SetColorAll(lights, color.Red, color.Green, color.Blue);
    }

    /// <summary>
    /// Sets the brightness of a specific light in the frame.
    /// </summary>
    /// <param name="light">The light to set the brightness for.</param>
    /// <param name="brightness">The brightness value to set. Must be between 0 and 100.</param>
    /// <exception cref="ArgumentOutOfRangeException">Thrown when the brightness value is outside the valid range.</exception>
    public void SetBrightness(Light light, int brightness) {
        if (brightness < 0 || brightness > 100) {
            throw new ArgumentOutOfRangeException(nameof(brightness), "Brightness must be between 0 and 255");
        }
        unsafe {
            NativeMethods.frame_set_brightness(ptr, light.ptr, (byte)brightness);
        }
    }

    /// <summary>
    /// Sets the brightness of all the lights in the specified array.
    /// </summary>
    /// <param name="lights">The array of lights.</param>
    /// <param name="brightness">The brightness value to set (between 0 and 100).</param>
    /// <exception cref="ArgumentOutOfRangeException">Thrown when the brightness value is outside the valid range.</exception>
    public void SetBrightnessAll(Light[] lights, int brightness) {
        if (brightness < 0 || brightness > 100) {
            throw new ArgumentOutOfRangeException(nameof(brightness), "Brightness must be between 0 and 255");
        }
        unsafe {
            foreach (var light in lights) {
                NativeMethods.frame_set_brightness(ptr, light.ptr, (byte)brightness);
            }
        }
    }

    /// <summary>
    /// Runs the frame asynchronously.
    /// </summary>
    /// <returns>A task representing the asynchronous operation.</returns>
    public Task Run() {
        return Task.Run(() => {
            unsafe {
                NativeMethods.frame_run(ptr);
            }
        });
    }

    /// <summary>
    /// Runs the frame synchronously.
    /// </summary>
    public void RunSync() {
        unsafe {
            NativeMethods.frame_run(ptr);
        }
    }

    /// <summary>
    /// Clears the frame.
    /// </summary>
     public void Clear() {
        unsafe {
            NativeMethods.frame_clear(ptr);
        }
    }

    /// <summary>
    /// Performs application-defined tasks associated with freeing, releasing, or resetting unmanaged resources.
    /// </summary>
    public void Dispose() {
        unsafe {
            NativeMethods.frame_free(ptr);
        }
    }

    ~Frame() {
        Dispose();
    }

}