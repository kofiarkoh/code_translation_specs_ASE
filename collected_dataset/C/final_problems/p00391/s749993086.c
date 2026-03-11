#include <stdio.h>

int col[1000], row[1000];

int main() {

    int i, j, s, t, w, h, sumC = 0, sumR = 0;
    scanf("%d %d", &w, &h);
    
    for ( i=0; i<w; ++i ) {
	scanf("%d", &col[i]);
	sumC += col[i];
    }
    for ( int i=0; i<h; ++i ) {
	scanf("%d", &row[i]);
	sumR += row[i];
    }

    if ( sumR != sumC ) {
	printf("0\n");
	return 0;
    }
    
    for ( i=0; i<w; ++i ) {

	for ( s=0; s<h; ++s ) {
	    for ( t=s+1; t<h; ++t ) {
		if ( row[s] < row[t] ) {
		    int tmp = row[s];
		    row[s] = row[t];
		    row[t] = tmp;
		}
	    }
	}

	for ( j=0; j<h; ++j ) {
	    if ( !col[i] || !row[j] ) break;
	    --row[j];
	    --col[i];
	}

	if ( col[i] > 0 ) {
	    printf("0\n");
	    return 0;
	}
    }
    printf("1\n");
    
    return 0;
}

