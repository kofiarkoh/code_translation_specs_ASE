#include<stdio.h>
#include<stdbool.h>
int d[] = {[0 ... 1<<18] -1};
int find(int x){
	if(d[x] < 0) return x;
	return d[x] = find(d[x]);
}
bool unite(int x, int y){
	x = find(x); y = find(y);
	if(x == y)return false;
	if(d[x] > d[y]){
		int tmp = x;
		x = y;
		y = tmp;
	}
	d[x] += d[y];
	d[y] = x;
	return true;
}
bool same(int x, int y){return find(x) == find(y);}
int Q, t, u, v;
int main(){
	scanf("%*d%d", &Q);
	while(Q--){
		scanf("%d%d%d", &t, &u, &v);
		if(t){
			printf("%d\n", same(u, v));
		}else {
			unite(u, v);
		}
	}
}