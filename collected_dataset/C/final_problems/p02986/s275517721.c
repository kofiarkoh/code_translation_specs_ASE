#include <stdio.h>
#define min(x,y) (x<y)?x:y
#define max(x,y) (x>y)?x:y
#define BIG 9999999
typedef struct{
	int m;
	int md;
	int mc;
	int d;
} tree;
tree t[100001];
int find_d(int node){
	if(t[node].d == -1){
		t[node].d = find_d(t[node].m)+1;
	}
	return t[node].d;
}
int main(void) {
	int N,Q,a[4],x[4],km,tem[3] = {};
	for(int i=0; i<100001; i++){
	  t[i].m  = 0;
	  t[i].md = 0;
	  t[i].mc = 0;
	  t[i].d  = 0;
	}
	scanf("%d%d",&N,&Q);
	t[1].m = BIG;
	t[1].d = 0;
	t[1].md = 0;
	for(int i=0; i<N-1; i++){
		scanf("%d%d%d%d",&a[0],&a[1],&a[2],&a[3]);
		if(t[a[0]].m == 0){
			t[a[0]].m = a[1];
			t[a[0]].mc = a[2];
			t[a[0]].md = a[3];
		}else{
			t[a[1]].m = a[0];
			t[a[1]].mc = a[2];
			t[a[1]].md = a[3];
		}
	}
	for(int i=2; i<=N; i++) t[i].d = -1;
	for(int i=2; i<=N; i++) find_d(i);
	for(int i=0; i<Q; i++){
		scanf("%d%d%d%d",&x[0],&x[1],&x[2],&x[3]);
		int ans = 0,tem1,tem2;
		tem1 = (t[x[2]].d>=t[x[3]].d)?x[2]:x[3];
		tem2 = (t[x[2]].d<t[x[3]].d)?x[2]:x[3];
		while(t[tem1].d != t[tem2].d) tem1 = t[tem1].m;
		while(t[tem1].m != t[tem2].m){
			tem1 = t[tem1].m;
			tem2 = t[tem2].m;
		}
		km = 1;
		if(tem1 != 1) km = t[tem1].m;
		while(x[2] != 1){
			if(t[x[2]].mc == x[0]){
				tem[0] += x[1];
			}else{
				tem[0] += t[x[2]].md;
			}
			x[2] = t[x[2]].m;
		}
		while(x[3] != 1){
			if(t[x[3]].mc == x[0]){
				tem[1] += x[1];
			}else{
				tem[1] += t[x[3]].md;
			}
			x[3] = t[x[3]].m;
		}
		while(km != 1){
			if(t[km].mc == x[0]){
				tem[2] += x[1];
			}else{
				tem[2] += t[km].md;
			}
			km = t[km].m;
		}
		ans = tem[0]+tem[1]-2*tem[2];
		printf("%d\n",ans);
		ans = 0;
		tem[0] = 0;
		tem[1] = 0;
		tem[2] = 0;
	}
	return 0;
}
