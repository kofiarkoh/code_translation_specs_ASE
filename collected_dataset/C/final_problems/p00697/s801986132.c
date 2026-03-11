#include <stdio.h>
#include <string.h>

int flg[9];

struct P{
	char s[8];
}p[9],set[9];

int solve(int idx){
	int i,j,k,res=0;

	if(idx == 9) return 1;

	for(k=0;k<9;k++){
		if(flg[k] == 1) continue;
		flg[k] = 1;
		for(i=0;i<4;i++){
			for(j=0;j<4;j++){
				set[idx].s[(i+j)%4] = p[k].s[j];
			}

			if(idx%3>0 &&
				 set[idx].s[3] + 32 != set[idx-1].s[1] &&
				 set[idx].s[3] - 32 != set[idx-1].s[1])
				continue;
			if(idx>2 &&
				 set[idx].s[0] + 32 != set[idx-3].s[2] &&
				 set[idx].s[0] - 32 != set[idx-3].s[2])
				continue;

			res += solve(idx+1);
		}
		flg[k] = 0;
	}

	return res;
}

int main(void){
	int i,n;

	scanf("%d",&n);

	while(n--){
		for(i=0;i<9;i++){
			scanf("%s",p[i].s);
		}
		memset(flg,0,sizeof(flg));
		printf("%d\n",solve(0));
	}

	return 0;
}