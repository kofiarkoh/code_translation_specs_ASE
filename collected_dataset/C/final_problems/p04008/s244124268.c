#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define lli long long int
#define min(a,b) ((a)<(b)?(a):(b))
#define max(a,b) ((a)>(b)?(a):(b))
#define true 1
#define false 0

typedef struct node {
	struct node* below;
	struct node* side;
	int isParent;
	int id;
} node;

void add(node *a,node *b){
	node *swap;
	swap = a->below;
	a->below = b;
	b->side = swap;
}

int dfs(node *a,lli k){
	static int counter;
	int height = 0;
	int c=0;
	node* b = a;
	a = a->below;
	while(a){
		counter = 0;
		c += dfs(a,k);
		a = a->side;
		height = max(counter,height);
	}
	counter++;
	if(height == k-1 && !b->isParent){
		b->isParent = true;
		c++;
		counter = 0;
	}

	return c;
}


int main(void){
	int i;
	int n;
	int x;
	int ans;
	lli k;
	node *a;
	scanf("%d%lld",&n,&k);
	a = calloc(sizeof(node),n);
	scanf("%d",&x);
	if(x==1){
		ans = 0;
	}else{
		ans = 1;
	}
	a[0].isParent = true;
	for(i=1;i<n;i++){
		scanf("%d",&x);
		add(&a[x-1],&a[i]);
		if(x==1){
			a[i].isParent = true;
		}
		a[i].id = i;
	}
	printf("%d\n",ans + dfs(a,k));

	return 0;
}