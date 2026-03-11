#include<stdio.h>
#define N 100000

int a[N];
int ans;
int n;


void check(int a[],int x,int mid,int com){
	int i;
	
	if(com==1){/*down*/
		for(i=mid;i>=0;i--){
			if(a[i] < x){
				ans=i+1;
				return;
			}
		}
	}
	else {/*up*/
		for(i=mid;i<=n-1;i++){
			if(a[i] > x){
				ans=i;
				return;
			}
		}
	}

}
		


void lower_bound(int a[],int x,int start,int last,int past){
	int mid=(last+start)/2;
	
	if(start > last)return;
	
	if(a[mid]==x){
		for(int i=mid-1;i>=0;i--){
			if(a[i]!=x){
				ans=i+1;
				return;
			}
		}
	}
	
	if(past==1){
		if(a[mid] < x)lower_bound(a,x,mid+1,last,1);
		else check(a,x,mid,1);
	}
	
	else if(past==-1){
		if(a[mid] > x)lower_bound(a,x,start,mid-1,-1);
		else check(a,x,mid,-1);
	}
	
	else {
		if(a[mid] < x)lower_bound(a,x,mid+1,last,1);
		if(a[mid] > x)lower_bound(a,x,start,mid-1,-1);
	}
	
	return;
	
}

int main(){
	int q,i,x;
	
	scanf("%d",&n);
	for(i=0;i<n;i++)scanf("%d",&a[i]);
	
	scanf("%d",&q);
	for(i=0;i<q;i++){
		scanf("%d",&x);
		if(x==0)ans=0;
		else if(x > a[n-1])ans=n;
		else lower_bound(a,x,0,n-1,0);
		printf("%d\n",ans);
		
	}
	
	return 0;
	
}
