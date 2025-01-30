#include <stdio.h>

int
main (void)
{
	// we'll find the n-th term of an arithmetic sequence
	// basically, implement this formula:
	// \f[a_n = a + (n - 1) \times d\f]
	float a = 1.f;
	float d = 2.f; // the distance in the sequence

	unsigned int n = 0;
	printf ("Please enter the n-th value: ");
	scanf ("%d", &n);

	float a_n = a + (n - 1) * d;
	printf ("The n-th term is: %f\n", a_n);

	// now, we'll calculate the sum of the sequence
	// we'll implement this formula:
	//\f[S_n = (a_1 + a_n) \times \frac{n}{2}\f]
	float sum = (a + a_n) * ((float)n / 2);
	printf ("The sum of the sequence is: %f\n", sum);

	return 0;
}
