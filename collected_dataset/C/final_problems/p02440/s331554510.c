#include<stdio.h>
#include<math.h>
#include<limits.h>
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define N 1000

int main(){
	int n,i,q,com,x,y,j;
	int a[N];
	int mini[N];
	int maxi[N];
	int b;
	int min=INT_MAX;
	int max=INT_MIN;
	
	scanf("%d",&n);
	b=sqrt(n);
	
	for(i=0;i<n;i++){
		scanf("%d",&a[i]);
		min=MIN(min,a[i]);
		max=MAX(max,a[i]);
		
		if(i%b==b-1){
			mini[i/b]=min;
			maxi[i/b]=max;
			min=INT_MAX;
			max=INT_MIN;
		}
	}
	
	scanf("%d",&q);
	for(i=0;i<q;i++){
		scanf("%d %d %d",&com,&x,&y);
		
		if(com){
			max=INT_MIN;
			if((y-1)/b-x/b <=1){
				for(j=x;j<=y-1;j++)max=MAX(max,a[j]);
			}
			else {
				for(j=x;j<=MIN(y-1,(x/b+1)*b-1);j++)max=MAX(max,a[j]);
				
				for(j=x/b+1;j<(y-1)/b;j++)max=MAX(max,maxi[j]);
			
				for(j=MAX(x,(y-1)/b*b);j<=y-1;j++)max=MAX(max,a[j]);
			}
			
			printf("%d\n",max);
		}
		
		else {
			min=INT_MAX;
			if((y-1)/b-x/b <=1){
				for(j=x;j<=y-1;j++)min=MIN(min,a[j]);
			}
			else {
				for(j=x;j<=MIN(y-1,(x/b+1)*b-1);j++)min=MIN(min,a[j]);
				
				for(j=x/b+1;j<(y-1)/b;j++)min=MIN(min,mini[j]);
			
				for(j=MAX(x,(y-1)/b*b);j<=y-1;j++)min=MIN(min,a[j]);
			}
			
			printf("%d\n",min);
		}
	}
	
	return 0;
}
