#include<stdio.h>
#include<stdlib.h>
#include <string.h>
#include <math.h>
#define NIL -1

const int MAX_N = 500005;

struct point{
	int x, y, id;
};

struct _node{
	struct point p;
	struct _node *left;
	struct _node *right;
};

struct rect{
	int x1, y1, x2, y2;
};


typedef struct _node * node;

struct point p[500005];

node head, z;
int ptr;
int a[500005];
int temp[500005];

void tree2Dinsert (struct point p ){

	node f, t;
	int d, td;

	for (d = 0, t = head; t != z; d = !d ){
		td = (d ? (p.x < t->p.x ) : (p.y < t->p.y ) );
		f = t;
		t = (td ? t->left : t->right );
	} // end for
	
	t = malloc (sizeof (struct _node ) );
	t->p = p; t->left = z; t->right = z;

	if (td )
		f->left = t;
	else
		f->right = t;
}

void preprocess (int n ){

	int i;
	p[0].x = 0; p[0].y = 0; p[0].id = -1;

	z = malloc (sizeof (struct _node ) );
	z->left = z; z->right = z; z->p = p[0];

	head = malloc (sizeof (struct _node ) );
	head->left = z; head->right = z; head->p = p[0];

	for (i = 1; i <= n; i++ ) tree2Dinsert (p[i] );
}

int insiderect (struct point p, struct rect range ){

	return ((range.x1 <= p.x && p.x <= range.x2 )
		&& (range.y1 <= p.y && p.y <= range.y2 ) );
}

int handle (struct point p ){
	a[ptr] = p.id;
	ptr++;
}

void tree2drange (node t, struct rect range, int d ){
	int t1, t2, tx1, tx2, ty1, ty2;
	if (t == z ) return;
	tx1 = (range.x1 < t->p.x );
	tx2 = (t->p.x <= range.x2 );
	ty1 = (range.y1 < t->p.y );
	ty2 = (t->p.y <= range.y2 );
	t1  = (d ? tx1 : ty1 );
	t2  = (d ? tx2 : ty2 );
	if (t1 ) tree2drange (t->left, range, !d );
	if (insiderect (t->p, range ) )
		if (t != head ) handle (t->p );
	if (t2 ) tree2drange (t->right, range, !d );
}

void bucket_sort (int radix ){

	int i, j, k;
	memset (temp, 0, sizeof (temp ) );
	int b[10][100];
	for (i = 0; i < 10; i++ ) for (j = 0; j < 100; j++ )
		b[i][j] = NIL;

	for (i = 0; i < ptr; i++ ){
		for (j = 0; j < 100; j++ ){
			if (b[a[i]/(int)pow(10, radix )][j] == NIL ){
				b[a[i]/(int)pow(10, radix )][j] = a[i];
				break;
			} // end if
		} // end for
	} // end rep

	k = 0;
	for (i = 0; i < 10; i++ ){
		for (j = 0; j < 100; j++ ){
			if (b[i][j] != NIL ){
				temp[k] = b[i][j];
				k++;
			}else{
				break;
			} // end if
		} // end rep
	} // end rep

	for (i = 0; i < ptr; i++ ) a[i] = temp[i];		
}

void radix_sort (void ){

	int i, maxn = 0;
	for (i = 0; i < ptr; i++ ){
		if (a[i] > maxn ) maxn = a[i];
	} // end rep

	for (i = 0; maxn > 0; i++, maxn /= 10 ){
		bucket_sort (i);
	} // end for
}


int main()
{
	int i, j, n, q, x, y, sx0, tx0, sy0, ty0;

	for (i = 0; i < MAX_N; i++ ) p[i].x = 0, p[i].y = 0, p[i].id = 0;

	scanf ("%d", &n );
	for (i = 1; i <= n; i++ ){
		scanf ("%d %d", &x, &y );
		p[i].x = x, p[i].y = y; p[i].id = i-1;
	} // end rep
	preprocess (n );

	scanf ("%d", &q );
	for (i = 0; i < q; i++ ){
		memset (a, 0, sizeof (a ) );
		ptr = 0;
		scanf ("%d %d %d %d", &sx0, &tx0, &sy0, &ty0 );
		struct rect r; r.x1 = sx0, r.y1 = sy0, r.x2 = tx0, r.y2 = ty0;
		tree2drange (head, r, 0 );
		radix_sort ();
		for (j = 0; j < ptr ;j++ ){
			printf ("%d\n", a[j] );
		} // end for
		printf ("\n" );
	} // end rep
		
	return 0;
}