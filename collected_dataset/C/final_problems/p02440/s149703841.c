#include<stdio.h>
#include<stdlib.h>
#define Max(a,b) a>b?a:b
#define Min(a,b) a<b?a:b

int min_find(int *a, int b, int e){
	int i;
	int ans=1000000001;
	for(i=0;i<e;i++){
		ans = Min(ans,a[i+b]);
	}
	return ans;
}

int max_find(int *a, int b, int e){
	int i;
	int ans=-1000000001;
	for(i=0;i<e;i++){
                ans = Max(ans,a[i+b]);
        }
	return ans;
}

typedef enum com{
	minimum,
	maximum
} Com;

int main(int argc, char *argv[]){
	int n, q, b,e;
	int i;
	int *a;
	Com com_x;
	int ans;

	scanf("%d",&n);
	a=calloc(n,sizeof(int));
	for(i=0;i<n;i++){
		scanf("%d",&a[i]);
	}

	scanf("%d",&q);
	for(i=0;i<q;i++){
		scanf("%d%d%d",&com_x,&b,&e);
		if(com_x==minimum){
			ans=min_find(a,b,e);
		}else if(com_x==maximum){
			ans=max_find(a,b,e);
		}
		printf("%d\n",ans);
	}

	exit(0);
}
