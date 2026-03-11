#include <stdio.h>
#include <math.h>
int q_sort(int d[],int left,int right)
{
	if(left >= right) return 0;
	int pipot = d[(left+right)/2],l = left, r = right,tmp;
	while(l < r){
		while(d[l] < pipot) l++;
		while(d[r] > pipot) r--;
		if(l <= r){
			tmp = d[l]; d[l] = d[r]; d[r] = tmp;
			l++; r--;
		}
	}
	q_sort(d,left,r);
	q_sort(d,l,right);
}
int divisor(int num,int d[])
{
	int i,j=0,c[(int)sqrt(num)],k;
	int castle=0;
	//printf("sqrt %d\n",(int)sqrt(num));
	for(i=1;i<=(int)sqrt(num);i++)
	{
		if(num % i == 0)
		{
			d[j] = i;
			castle++;
			if(i != num / i) c[j] = num/i;
			j++;
			castle++;
		}
	}
	k = j;j--;
	for(i = j; i >= 0; i--){
		d[k] = c[i];
		k++;
	}
	return castle;
}




int kate(int ko_ho,int d[],int K,int N)
{
	int i,j,c[N],w = 0;
	for(i = 0; i < N; i++)
	{
		c[i] = d[i]%ko_ho;
		if(c[i] == 0) w++;
	}
	q_sort(c,0,N-1);
	int plas= 0, mainus=0;
	/*printf("%d\n",ko_ho);
	for(i = 0 ; i < N; i++)
	{
		printf("%d ",c[i]);
	}
	printf("\n");
	*/
	for(i = w; i < N; i++)
	{
		plas = 0; mainus = 0;
		for( j = 0; j < i; j++)
		{
			mainus+=c[j];
		}
		for( j = i ; j < N; j++)
		{
			plas += ko_ho - c[j];
		}
		if(plas==mainus) break;
		//printf("i = %d, plas = %d, mainus = %d\n",i,plas,mainus);
	}
	if(plas <= K) return 1;
	else return 0;
}






int main(void)
{
	int N,K;
	scanf("%d %d",&N,&K);
	int A[N],i,sum=0;
	for(i = 0; i < N ; i++)
	{
		scanf("%d",&A[i]);
		sum += A[i];
	}
	
	int kouho[2*(int)sqrt(sum)],c;
	c = divisor(sum,kouho);
	
	/*
	for(i = 0 ; i < 10; i++)
	{
		printf("ap %d ",kouho[i]);
	}
	printf("\n");
	*/
	int e = 0,r;
	for(i = c-1; i >= 0; i--)
	{
		r = kouho[i];
		e = kate(r,A,K,N);
		if(e == 1) break;
	}
	printf("%d",r);
	//*/
	return 0;
}

