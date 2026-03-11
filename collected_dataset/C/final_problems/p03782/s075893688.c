#include <stdio.h>
#include <stdlib.h>
#define ll long long
#define FOR(i,a,n) for(int i=a;i<n;i++)

int comp(const void* a,const void* b){
	return *(int*)b-*(int*)a;
}
int ary[5000];
 int main(void)
 {
 	int n,k,a,begin=0,end=0,p=0;
 	int num=0;
 	scanf("%d %d",&n,&k);
 	for(int i=0;i<n;i++){
 		scanf("%d",&a);
 		if(a<k) ary[num++]=a;
 	}
     qsort(ary,num,sizeof(int),comp);
     int sum=ary[0];
     while(end<num){
     	while(end<num&&(sum<k||ary[end]==ary[end+1])) sum+=ary[++end];
     	if(end==num) break;
     	p=end+1;
     	sum-=ary[begin++];
     }
     printf("%d\n",sum>=k?0:end-p);
 	return 0;
 }
 
    