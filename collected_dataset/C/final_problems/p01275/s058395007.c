#include <stdio.h>
#include <stdlib.h>

char s[12],t[12];
int rotate[10][10];
int n,ans;

void solve(int idx,int cost){
	if(idx == n) ans = cost;
	if(cost >= ans) return;

	int i,j,r;

	if(s[idx] == t[idx]){
		solve(idx+1,cost);
	}
	else{
		for(i=idx;i<n;i++){
			r = rotate[s[idx]][t[idx]];
			for(j=idx;j<=i;j++) s[j] = (s[j] + 10 + r) % 10;
			solve(idx+1,cost+1);
			for(j=idx;j<=i;j++) s[j] = (s[j] + 10 - r) % 10;
		}
	}
}

int main(void){
	int i,j,d,x;

	for(i=0;i<10;i++){
		for(j=0;j<10;j++){
			d = j - i;
			x = abs(d);
			if(j < i && x > 10-x)
				d = 10-x;
			else if(j > i && x > 10-x)
				d = -(10-x);

			rotate[i][j] = d;
		}
	}

	while(scanf("%d",&n),n){
		scanf("%s%s",s,t);
		for(i=0;i<n;i++){
			s[i] -= '0';
			t[i] -= '0';
		}

		ans = 999999999;
		solve(0,0);
		printf("%d\n",ans);
	}

	return 0;
}