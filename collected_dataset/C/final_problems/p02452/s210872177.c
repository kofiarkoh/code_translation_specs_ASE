#include<stdio.h>
#define N 200000

int a[N];

int bin_search(int start,int last,int key){
	int mid=(last-start)/2+start;
	
	if(start > last)return 0;
	
	if(a[mid]==key)return 1;
	
	if(key > a[mid])return bin_search(mid+1,last,key);
	else return bin_search(0,mid-1,key);
	
}



int main(){
	int n,m,q,i,x;
	int flg=1;
	
	scanf("%d",&n);
	for(i=0;i<n;i++)scanf("%d",&a[i]);
	
	scanf("%d",&m);
	for(i=0;i<m;i++){
		scanf("%d",&x);
		if(!bin_search(0,n-1,x))flg=0;
	}
	
	printf("%d\n",flg);
	
	return 0;
	
}
