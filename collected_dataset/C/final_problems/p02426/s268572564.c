#include <stdio.h>
#include <stdlib.h>
#include <math.h>


#define	SIZE	64


void Init(int *box)
{
	int i;

	for(i = 0; i < SIZE; i++)
		box[i]  = 0;
}


void Test(int *box, int i)
{
	if(box[i])
		printf("1\n");
	else
		printf("0\n");
}


void Set(int *box, int **mask_box)
{
	int x;
	int i = 0;

	while( (x = mask_box[0][i]) != -1){
		box[x] = 1;
		i++;
	}
}


void Clear(int *box, int **mask_box)
{
	int x;
	int i = 0;

	while( (x = mask_box[0][i]) != -1){
		box[x] = 0;
		i++;
	}
}


void Flip(int *box, int **mask_box)
{
	int x;
	int i = 0;

	while( (x = mask_box[0][i]) != -1){
		if(box[x])
			box[x] = 0;
		else
			box[x] = 1;
		i++;
	}
}


void All(int *box, int **mask_box)
{
	int x;
	int i = 0;

	while( (x = mask_box[0][i]) != -1){
		if(box[x] == 0){
			printf("0\n");
			return;
		}
		i++;
	}

	printf("1\n");
}


void Any(int *box, int **mask_box)
{
	int x;
	int i = 0;

	while( (x = mask_box[0][i]) != -1){
		if(box[x]){
			printf("1\n");
			return;
		}
		i++;
	}

	printf("0\n");
}


void None(int *box, int **mask_box)
{
	int x;
	int i = 0;

	while( (x = mask_box[0][i]) != -1){
		if(box[x]){
			printf("0\n");
			return;
		}
		i++;
	}

	printf("1\n");
}


void Count(int *box, int **mask_box)
{
	int count = 0;
	int x;
	int i = 0;

	while( (x = mask_box[0][i]) != -1){
		if(box[x])
			count++;
		i++;
	}

	printf("%d\n", count);
}


void Val(int *box, int **mask_box)
{
	unsigned long long int sum = 0;
	int x;
	int i = 0;

	while( (x = mask_box[0][i]) != -1){
		sum += (unsigned long long int)(pow(2, x) * box[x]);
		i++;
	}

	printf("%llu\n", sum);
}


int main()
{
	int n, size;
	int q, command, m;
	int **mask_box, *_mask_box;
	int bit_box[SIZE];
	int i, j;

	scanf("%d", &n);

	_mask_box = (int *)malloc(sizeof(int) *  n * (SIZE + 1));
	mask_box = (int **)malloc(sizeof(int *) * n);
	for(i = 0; i < n; i++)
		mask_box[i] = _mask_box + (i * (SIZE + 1));

	for(i = 0; i < n; i++){
		scanf("%d", &size);
		for(j = 0; j < size; j++)
			scanf("%d", &mask_box[i][j]);
		mask_box[i][size] = -1;
	}

	scanf("%d", &q);

	Init(bit_box);

	for(i = 0; i < q; i++){
		scanf("%d", &command);

		switch(command){
			case 0:
			// test
				scanf("%d", &m);
				Test(bit_box, m);
				break;
			case 1:
			//set
				scanf("%d", &m);
				Set(bit_box, mask_box + m);
				break;
			case 2:
			// clear
				scanf("%d", &m);
				Clear(bit_box, mask_box + m);
				break;
			case 3:
			// flip
				scanf("%d", &m);
				Flip(bit_box, mask_box + m);
				break;
			case 4:
			// all
				scanf("%d", &m);
				All(bit_box, mask_box + m);
				break;
			case 5:
			// any
				scanf("%d", &m);
				Any(bit_box, mask_box + m);
				break;
			case 6:
			// none
				scanf("%d", &m);
				None(bit_box, mask_box + m);
				break;
			case 7:
			// count
				scanf("%d", &m);
				Count(bit_box, mask_box + m);
				break;
			case 8:
			// val
				scanf("%d", &m);
				Val(bit_box, mask_box + m);
				break;

		}
	}

	return 0;
}
