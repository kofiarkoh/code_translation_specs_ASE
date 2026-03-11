#include<stdio.h>
#include<stdlib.h>
#define MAX 200000
#define INF 2000000000

int compare_int(const void *a,const void *b){
	return *(int*)a-*(int*)b;
}

int main(){
	int n,m;
	int a[MAX+1];
	int b[MAX+1];
	int j=0,k=0;
	
	scanf("%d",&n);
	for(int i=0;i<n;i++)
		scanf("%d",&a[i]);
	a[n]=INF;
	
	qsort(a,n,sizeof(int),compare_int);
	
	scanf("%d",&m);
	for(int i=0;i<m;i++)
		scanf("%d",&b[i]);
	b[m]=INF;
	
	qsort(b,m,sizeof(int),compare_int);
	
	for(int i=0;i<n+m;i++){
	
		if(a[j]==INF || b[k]==INF)break;
		
		if(a[j] == b[k]){
			printf("%d\n",a[j]);
			int temp=a[j];
			while(a[j]==temp)j++;
			while(b[k]==temp)k++;
		}
		
		else if(a[j] < b[k])
			j++;
			
		else if(a[j] > b[k])
			k++;
		
		
	}
	
	return 0;
	
}
