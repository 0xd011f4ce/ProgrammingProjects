#include <stdio.h>

int
main (void)
{
	double celsius_temp = 0.0, fahrenheit_temp = 0.0;
	printf ("Please enter a temperature (in celsius): ");
	scanf ("%lf", &celsius_temp);

	fahrenheit_temp = (celsius_temp * 1.8) + 32;
	printf ("The temperature in fahrenheit degrees is: %.2lf\n",
					fahrenheit_temp);

	printf ("Enter a temperature in fahrenheit degrees: ");
	scanf ("%lf", &fahrenheit_temp);

	celsius_temp = (fahrenheit_temp - 32) / 1.8;
	printf ("The temperature in celsius is: %lf\n", celsius_temp);

	return 0;
}
