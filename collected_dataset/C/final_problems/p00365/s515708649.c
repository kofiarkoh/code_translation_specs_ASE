#include<stdio.h>

void swap( int* a, int* b ) { int t = *a; *a = *b; *b = t; }

int main() {

    int y1, m1, d1, y2, m2, d2;
    scanf("%d %d %d %d %d %d", &y1, &m1, &d1, &y2, &m2, &d2 );

    if ( y1 > y2 || (y1 == y2 && (m1 > m2 || (m1 == m2 && d1 > d2))) ) {
	swap( &y1, &y2 );
	swap( &m1, &m2 );
	swap( &d1, &d2 );
    }
    
    if ( m1 < m2 || (m1 == m2 && d1 < d2 ) ) printf("%d\n", y2 - y1 + 1);
    else printf("%d\n", y2 - y1);
    
    return 0;
}

