#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct ARR {
	int data;
	int b;
}arr;

void insert(arr *L, int x, int q, int *cursor, int *first) {
    int i;
	if ((*cursor)!=(*first)) {
		for (i=(*first);i<(*cursor);i++) L[i-1]=L[i];
	}
		L[(*cursor)-1].data=x;
		L[(*cursor)-1].b=1;
		(*cursor)--;
		(*first)--;
}

void move(arr *L, int d, int *cursor) {
    //(*cursor)+=d;
	if (d>0) {
		do {
			if (L[(*cursor)].b==0) {
				(*cursor)++;
			} else {
				(*cursor)++;
				d--;
			}
		} while (d>0);
	} else {
		do {
			if (L[(*cursor)-1].b==0) {
				(*cursor)--;
			} else {
				(*cursor)--;
				d++;
			}
		} while (d<0);
	}
}

void erase (arr *L, int q, int *cursor, int *first) {
    // int i;
    // for (i=*cursor; i>*first; i--) L[i]=L[i-1];
	if (*cursor==*first) {
		L[(*cursor)].b=0;
		while (L[(*cursor)].b==0) (*cursor)++, (*first)++;

	} else {
		L[(*cursor)].b=0;
		while (L[(*cursor)].b==0) (*cursor)++;
	}
    // L[(*first)++]=-1;
}

int main (void) {
    int i, q, cursor, query, first;
	int x;
	arr *L;
	// int j;

    scanf("%d", &q);
    L=(arr *)malloc(sizeof(arr)*q+1);
    //memset(L.b, 0, sizeof(int)*q+1);
    for (i=0; i<q+1; i++) L[i].b=0;
	cursor=q+1;
    first=q+1;
	L[q+1].b=1;
    for (i=0; i<q; i++) {
        scanf("%d", &query);
        if (query==0) {
            scanf("%d", &x);
            insert (L, x, q, &cursor, &first);
        } else if (query==1) {
            scanf("%d", &x);
            move(L, x, &cursor);
        } else {
            erase(L, q, &cursor, &first);
        }
		/*
		printf("first:%d cursor:%d\narr:", first, cursor);
		for (j=0;j<q+1;j++) {
	        if (L[j].b==0) continue;
	        printf("%d", L[j].data);
			if (i<q+1) printf(" ");
	    }
		printf("\n");
		*/
    }
    for (i=first;i<q+1;i++) {
        if (L[i].b==0) continue;
        printf("%d\n", L[i].data);
    }
    return 0;
}

