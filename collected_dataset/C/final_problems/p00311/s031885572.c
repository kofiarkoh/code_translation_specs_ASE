#include<stdio.h>

int h1, h2;
int k1, k2;
int a, b, c, d;

int main(void)
{
	int p1, p2;
	
	scanf( "%d %d %d %d %d %d %d %d", &h1, &h2, &k1, &k2, &a, &b, &c, &d );
	
	p1 = h1 * a + h2 * b + (h1 / 10) * c + (h2 / 20) * d;
	p2 = k1 * a + k2 * b + (k1 / 10) * c + (k2 / 20) * d;
	
	if ( p1 > p2 ) printf( "hiroshi\n" );
	else if ( p1 < p2 ) printf( "kenjiro\n" );
	else printf( "even\n" );
	
	return 0;
}