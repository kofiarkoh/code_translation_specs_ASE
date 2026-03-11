#include <stdio.h>
#include <math.h>


#define	SIZE	64


void Init(int *box)
{
	int i;

	for(i = 0; i < SIZE; i++)
		box[i]  = 0;
}


void Test(int *box, int x)
{
	if(box[x])
		printf("1\n");
	else
		printf("0\n");
}


void Set(int *box, int x)
{
	box[x] = 1;
}


void Clear(int *box, int x)
{
	box[x] = 0;
}


void Flip(int *box, int x)
{
	if(box[x])
		box[x] = 0;
	else
		box[x] = 1;
}


void All(int *box)
{
	int i;

	for(i = 0; i < SIZE; i++){
		if(box[i] == 0){
			printf("0\n");
			return;
		}
	}

	printf("1\n");
}


void Any(int *box)
{
	int i;

	for(i = 0; i < SIZE; i++){
		if(box[i]){
			printf("1\n");
			return;
		}
	}

	printf("0\n");
}


void None(int *box)
{
	int i;

	for(i = 0; i < SIZE; i++){
		if(box[i]){
			printf("0\n");
			return;
		}
	}

	printf("1\n");
}


void Count(int *box)
{
	int count = 0;
	int i;

	for(i = 0; i < SIZE; i++)
		if(box[i])
			count++;

	printf("%d\n", count);
}


void Val(int *box)
{
	unsigned long long int sum = 0;
	int i;

	for(i = 0; i < SIZE; i++)
		sum += (unsigned long long int)(pow(2, i) * box[i]);

	printf("%llu\n", sum);
}


int main()
{
	int q, command, x;
	int bit_box[SIZE];
	int i;

	scanf("%d", &q);

	Init(bit_box);

	for(i = 0; i < q; i++){
		scanf("%d", &command);

		switch(command){
			case 0:
			// test
				scanf("%d", &x);
				Test(bit_box, x);
				break;
			case 1:
			//set
				scanf("%d", &x);
				Set(bit_box, x);
				break;
			case 2:
			// clear
				scanf("%d", &x);
				Clear(bit_box, x);
				break;
			case 3:
			// flip
				scanf("%d", &x);
				Flip(bit_box, x);
				break;
			case 4:
			// all
				All(bit_box);
				break;
			case 5:
			// any
				Any(bit_box);
				break;
			case 6:
			// none
				None(bit_box);
				break;
			case 7:
			// count
				Count(bit_box);
				break;
			case 8:
			// val
				Val(bit_box);
				break;
		}
	}

	return 0;
}
