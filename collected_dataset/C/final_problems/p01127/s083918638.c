#include <stdio.h>

#define REP(i,n,m) for(i=n;i<m;i++)
#define rep(i,n) REP(i,0,n)

int h,w;
int mt[26][4]; //[i][0]:¶[ [1]:E[ [2]:ã[ [3]:º[
int used[26];
char t[54][54];

//(a+'A') Ì¶ª (b+'B') Ì¶æèOÉ é©Ì»è
//»Ì»èÆ¯É, a ª·û`Å é©à»è·é
int isFront(int a,int b){
	int i,j;
	REP(i,mt[a][2],mt[a][3]+1){
		REP(j,mt[a][0],mt[a][1]+1){
			if(t[i][j] == b + 'A' || t[i][j] == '.'){
				return 0;
			}
		}
	}
	return 1;
}

//SafeÅ é©Ç¤©Ì»è
int isSafe(int idx,int rem){
	int i,j;
	int okFlg;

	if(rem == 0){
		//ÅãÌLª·û`©»è·é½ßÉisFrontðg¤
		return isFront(idx,-1);
	}

	used[idx] = 1;
	rep(i,26){
		if(mt[i][1]!=-1 && !used[i]){

			//¡ÜÅgpµ½LSÄæèãëÉ é©»è
			okFlg = 1;
			rep(j,26) if(used[j] && !isFront(j,i)){
				okFlg = 0;
				break;
			}

			if(okFlg && isSafe(i,rem-1)) return 1;
		}
	}
	used[idx] = 0;

	return 0;
}

int main(void){
	int i,j;
	int n,idx,count;

	scanf("%d",&n);
	while(n--){
		scanf("%d%d",&h,&w);

		rep(i,26){
			mt[i][0] = w+1;
			mt[i][1] = -1;
			mt[i][2] = h+1;
			mt[i][3] = -1;
		}

		count = 0;
		rep(i,h){
			scanf("%s",t[i]);
			rep(j,w) if(t[i][j] != '.') {
				idx = t[i][j] - 'A';

				//LÌíÞJEg
				if(mt[idx][1] == -1) count++;

				//LÌ¶[,E[,ã[,º[ðXV
				if(mt[idx][0] > j) mt[idx][0] = j;
				if(mt[idx][1] < j) mt[idx][1] = j;
				if(mt[idx][2] > i) mt[idx][2] = i;
				if(mt[idx][3] < i) mt[idx][3] = i;
			}
		}

		rep(i,26) used[i] = 0;
		rep(i,26) if(mt[i][1] != -1 && isSafe(i,count-1)) break;

		printf("%s\n",count!=0 && i==26 ? "SUSPICIOUS" : "SAFE");
	}

	return 0;
}