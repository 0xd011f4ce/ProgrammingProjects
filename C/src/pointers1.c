#include <stdio.h>

int
main (void)
{
	int grade1 = 80, grade2 = 100;

	printf ("grade1 = %d\n&grade1 = %p\n", grade1, &grade1);
	printf ("grade2 = %d\n&grade2 = %p\n", grade2, &grade2);

	return 0;
}
