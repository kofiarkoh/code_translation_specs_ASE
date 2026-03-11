#include <stdio.h>

#define INF 999999999
#define REP(i,n,m) for(i=n;i<m;i++)
#define rep(i,n) REP(i,0,n)

void printTable(int h,int w,int t[102][102],int *sales,int *totals){
	int i,j;
	rep(i,h){
		rep(j,w) printf("%10d",t[i][j]);
		printf("%10d\n",totals[i]);
	}
	rep(i,w+1) printf("%10d",sales[i]);
	printf("\n\n");
}

int main(void){
	int i,j;
	int h,w,tmp,contFlg,count;
	int flg[102][102],t[102][102],totals[102],sales[102],ct[102],cs[102];

	while(scanf("%d",&h) && h){
		scanf("%d",&w);

		rep(i,h+1) totals[i] = ct[i] = 0;
		rep(i,w+1) sales[i] = cs[i] = 0;

		rep(i,h){
			rep(j,w){
				tmp = scanf("%d",&t[i][j]);
				if(tmp == 0){
					ct[i]++;
					cs[j]++;
					flg[i][j] = 1;
					t[i][j] = INF;
					getchar();
				}
				else{
					flg[i][j] = 0;
					totals[i] -= t[i][j];
					sales[j] -= t[i][j];
				}
			}
			scanf("%d",&tmp);
			totals[i] += tmp;
		}
		rep(i,w+1){
			scanf("%d",&tmp);
			sales[i] += tmp;
		}

		contFlg = 1;
		while(contFlg){
			contFlg = count = 0;
			rep(i,h){
				rep(j,w){
					if(t[i][j] == INF){
						if(ct[i] == 1){
							t[i][j] = totals[i];
							sales[j] -= totals[i];
							totals[i] = 0;
							ct[i] = 0;
							cs[j]--;

							contFlg = 1;
						}
						else if(cs[j] == 1){
							t[i][j] = sales[j];
							totals[i] -= sales[j];
							sales[j] = 0;
							ct[i]--;
							cs[j] = 0;

							contFlg = 1;
						}
						else{
							count++;
						}
					}
				}
			}
		}

		if(count == 0){
			rep(i,h) rep(j,w) if(flg[i][j] && t[i][j] != INF) {
				printf("%d\n",t[i][j]);
			}
		}
		else{
			printf("NO\n");
		}
		printf("\n");
	}

	return 0;
}