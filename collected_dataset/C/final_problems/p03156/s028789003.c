#include <stdio.h>
#include <stdlib.h>
 
int compare(const void *a,const void *b){
	return *(int*)a-*(int*)b;
}
 
int main(){
	int N,A,B;
	int i;
	int first=0,second=0,third=0;
	int contest=0;
	
	scanf("%d",&N);
	scanf("%d %d",&A,&B);
	
	int Pi[N];
	
	for(i=0;i<N;i++){
		scanf("%d",&Pi[i]);
	}
	
	qsort(Pi,N,sizeof(int),compare);
	
	i=0;
	while(Pi[i]<A+1){
		first++;
		i++;
	}
	
	i=N-1;
	while(B<Pi[i]){
		third++;
		i--;
	}
	second=N-first-third;
	
	
	while(first!=0 && third!=0){
		contest++;
		
		first--;
		if(second==0)	first--;
		third--;
	}
	
	printf("%d",contest);
	return 0;
}