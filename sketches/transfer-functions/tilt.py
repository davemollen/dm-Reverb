from scipy import signal
import numpy as np
import matplotlib.pyplot as plt
from enum import Enum

# Set the sample rate
sample_rate = 44100  # in Hz

# Change the tilt value to see the difference in the frequency response
# Keep it between 0 and 1
tilt = 0.

def get_s_domain_coefficients(tilt):
  # The following transfer function was derived with QsapecNG:
  # ( ( - C1 * C2 * R2 * RF1 * RF2 ) + ( - C1 * C2 * R2 * RF2 * RTilt*(1-a) ) + ( - C1 * C2 * R1 * R2 * RTilt*(1-a) ) + ( - C1 * C2 * R1 * R2 * RF2 ) + ( - C1 * C2 * RF1 * RF2 * RTilt*a ) + ( - C1 * C2 * R2 * RF2 * RTilt*a ) ) * s^2 + ( ( - C1 * RF1 * RF2 ) + ( - C1 * RF2 * RTilt*(1-a) ) + ( - C2 * R2 * RTilt*(1-a) ) + ( - C2 * R2 * RF2 ) + ( - C1 * R1 * RTilt*(1-a) ) + ( - C1 * R1 * RF2 ) + ( - C1 * RF2 * RTilt*a ) ) * s + ( ( - RTilt*(1-a) ) + ( - RF2 ) )
  # ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
  # ( C1 * C2 * RF1 * RF2 * RTilt*(1-a) + C1 * C2 * R1 * RF1 * RTilt*(1-a) + C1 * C2 * R1 * RF1 * RF2 + C1 * C2 * R1 * R2 * RF1 + C1 * C2 * R1 * RF1 * RTilt*a + C1 * C2 * R1 * R2 * RTilt*a ) * s^2 + ( C2 * RF1 * RTilt*(1-a) + C2 * RF1 * RF2 + C2 * R2 * RF1 + C1 * R1 * RF1 + C2 * RF1 * RTilt*a + C2 * R2 * RTilt*a + C1 * R1 * RTilt*a ) * s + ( RF1 + RTilt*a )
  
  # This function implements this transfer function, but with less repeated calculations.
  
  c1 = 5.6e-9
  c2 = 5.6e-9
  r1 = 2250
  r2 = 2250
  rf1 = 47000
  rf2 = 47000
  r_tilt_a = 140000 * tilt
  r_tilt_b = 140000 * (1-tilt)

  c1c2 = c1 * c2
  c1c2r1 = c1c2 * r1
  c1c2r1r2 = c1c2r1 * r2
  c1c2r2 = c1c2 * r2
  c1c2r2rf2 = c1c2r2 * rf2
  c2r2 = c2 * r2
  c1r1 = c1 * r1
  c1rf2 = c1 * rf2
  c1c2rf1rf2 = c1c2 * rf1 * rf2
  c2rf1 = c2 * rf1

  b0 = -c1c2r2rf2 * rf1 + -c1c2r2rf2 * r_tilt_b + -c1c2r1r2 * r_tilt_b + -c1c2r2rf2 * r1 + -c1c2rf1rf2 * r_tilt_a + -c1c2r2rf2 * r_tilt_a
  b1 = -c1rf2 * rf1 + -c1rf2 * r_tilt_b + -c2r2 * r_tilt_b + -c2r2 * rf2 + -c1r1 * r_tilt_b + -c1r1 * rf2 + -c1rf2 * r_tilt_a
  b2 = -r_tilt_b + -rf2
  a0 = c1c2rf1rf2 * r_tilt_b + c1c2r1 * rf1 * r_tilt_b + c1c2rf1rf2 * r1 + c1c2r1r2 * rf1 + c1c2r1 * rf1 * r_tilt_a + c1c2r1r2 * r_tilt_a
  a1 = c2rf1 * r_tilt_b + c2rf1 * rf2 + c2r2 * rf1 + c1r1 * rf1 + c2rf1 * r_tilt_a + c2r2 * r_tilt_a + c1r1 * r_tilt_a
  a2 = rf1 + r_tilt_a

  return ([b0, b1, b2], [a0, a1, a2])

# Get the s-domain coefficients
num, den = get_s_domain_coefficients(tilt)
print("s-domain coefficients:", (num, den))

# Apply the bilinear transform
b, a = signal.bilinear(num, den, fs=sample_rate)
print("z-domain coefficients:", (list(b), list(a)))

# Get the frequency response
w,h = signal.freqz(b, a, 2**20)
w = w * sample_rate / (2 *np.pi)

# Plot the frequency response
fig = plt.figure()
plt.title('Digital filter frequency response')
plt.semilogx(w, 20 * np.log10(abs(h)), 'b')
plt.ylabel('magnitude [dB]')
plt.xlabel('frequency [Hz]')
plt.grid()
plt.axis('tight')
plt.xlim([1, 20000])
plt.ylim([-32, 32])
plt.show()
