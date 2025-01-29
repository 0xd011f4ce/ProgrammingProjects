#include <stdio.h>

int
main (void)
{
	float a = 2.f, b = 3.f, temp = 0.f;
	printf ("a = %f, b = %f\n", a, b);

	temp = b;
	b = a;
	a = temp;

	printf ("a = %f, b = %f\n", a, b);

	return 0;
}
