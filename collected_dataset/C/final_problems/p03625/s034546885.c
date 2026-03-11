#include<stdio.h>
#define MAX 100000

void sort(int *Ai, int N);
int pair_check(int *Ai, int N);

int main()
{
	int N;
	int Ai[MAX];
	int i;

	//入力
	scanf("%d", &N);
	for(i=0; i<N; i++)
	{
		scanf("%d", &Ai[i]);
	}
	
	sort(Ai, N);
	printf("%d\n", pair_check(Ai, N));

	return 0;
}

void sort(int *Ai, int N)
{
	int i, j;
	int temp;

	for(i=0; i<N; i++){
		for(j=i; j<N; j++){
			if(Ai[i] < Ai[j])
			{
				temp = Ai[i];
				Ai[i] = Ai[j];
				Ai[j] = temp;
			}
		}
	}
}

int pair_check(int *Ai, int N)
{
	int pair_num = 0;
	int j=0;
	int box[2] = {0};

	for(j=0; j+1<N; j++)
	{
		if(Ai[j] == Ai[j+1])
		{
			box[pair_num] = Ai[j];
			j++;
			if(++pair_num==2)break;
		}
	}

	return box[0]*box[1];
}
