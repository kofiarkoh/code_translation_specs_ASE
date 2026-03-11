#include <stdio.h>
#include <stdlib.h>


#define	ll_int	long long int


ll_int *WORK;

void Rotate(ll_int *box, int n, int b, int m, int e)
{
	int i;

	for(i = 0; i < e - b; i++)
		box[b + ((i + (e - m)) % (e - b))] = WORK[b + i];
}


void Print_box(ll_int *box, int n)
{
	int i;

	for(i = 0; i < n; i++){
		printf("%lld", box[i]);
		if(i != n - 1)
			printf(" ");
	}
	printf("\n");
}


int main()
{
	int n, q;
	int b, m, e;
	ll_int *box;
	int i, j;


	scanf("%d", &n);

	box = (ll_int *)malloc(sizeof(ll_int) * n);
	WORK = (ll_int *)malloc(sizeof(ll_int) * n);

	for(i = 0; i < n; i++)
		scanf("%lld", &box[i]);

	scanf("%d", &q);

	for(i = 0; i < q; i++){
		for(j = 0; j < n; j++)
			WORK[j] = box[j];

		scanf("%d %d %d", &b, &m ,&e);

		Rotate(box, n, b, m, e);
	}


	Print_box(box, n);

	return 0;
}
