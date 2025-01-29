#include <stdio.h>

int
main (void)
{
	float rectangle_width = 0.f;
	float rectangle_height = 0.f;

	printf ("Please enter the rectangle width: ");
	scanf ("%f", &rectangle_width);
	printf ("Please enter the rectangle height: ");
	scanf ("%f", &rectangle_height);

	printf ("The area of the rectangle is: %f\n",
					rectangle_width * rectangle_height);

	printf ("The perimeter of the rectangle is: %f\n",
					2 * (rectangle_width + rectangle_height));

	return 0;
}
