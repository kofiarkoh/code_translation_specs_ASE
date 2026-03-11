#include <stdio.h>
#include <string.h>

int h,w;
char nimotu[52][52];
int nimotu_data[27][52][52];
struct {
	int xmin,xmax;
	int ymin,ymax;
} nimotu_range[26];

int solve(void) {
	int i;
	int x,y;
	int kousinn_atta;
	if(scanf("%d%d",&h,&w)!=2)return 0;
	for(i=1;i<=h;i++) {
		if(scanf("%s",nimotu[i]+1)!=1)return 0;
	}
	memset(nimotu_data,0,sizeof(nimotu_data));
	for(i=0;i<26;i++) {
		nimotu_range[i].xmin=w+2;
		nimotu_range[i].xmax=-1;
		nimotu_range[i].ymin=h+2;
		nimotu_range[i].ymax=-1;
	}
	for(y=1;y<=h;y++) {
		for(x=1;x<=w;x++) {
			if(nimotu[y][x]!='.') {
				/* ASCII code wo katei */
				int m=nimotu[y][x]-'A';
				nimotu_data[m][y][x]++;
				if(nimotu_range[m].xmin>x)nimotu_range[m].xmin=x;
				if(nimotu_range[m].xmax<x)nimotu_range[m].xmax=x;
				if(nimotu_range[m].ymin>y)nimotu_range[m].ymin=y;
				if(nimotu_range[m].ymax<y)nimotu_range[m].ymax=y;
			}
		}
	}
	for(i=0;i<26;i++) {
		for(y=1;y<=h;y++) {
			for(x=1;x<=w;x++)nimotu_data[i][y][x]+=nimotu_data[i][y][x-1];
		}
		for(x=1;x<=w;x++) {
			for(y=1;y<=h;y++)nimotu_data[i][y][x]+=nimotu_data[i][y-1][x];
		}
	}
	do {
		kousinn_atta=0;
		for(i=0;i<26;i++) {
			if(nimotu_range[i].xmin<=nimotu_range[i].xmax) {
				int xs=nimotu_range[i].xmin-1;
				int xt=nimotu_range[i].xmax;
				int ys=nimotu_range[i].ymin-1;
				int yt=nimotu_range[i].ymax;
				int n=nimotu_data[i][yt][xt]-nimotu_data[i][yt][xs]
					-nimotu_data[i][ys][xt]+nimotu_data[i][ys][xs];
				int v=nimotu_data[26][yt][xt]-nimotu_data[26][yt][xs]
					-nimotu_data[26][ys][xt]+nimotu_data[26][ys][xs];
				if(n+v==(xt-xs)*(yt-ys)) {
					kousinn_atta=1;
					/* kono nimotu wo hazusu */
					nimotu_range[i].xmin=w+2;
					nimotu_range[i].xmax=-1;
					/* ruisekiwa wo kaizyo */
					for(x=1;x<=w;x++) {
						for(y=h;y>=1;y--) {
							nimotu_data[i][y][x]-=nimotu_data[i][y-1][x];
							nimotu_data[26][y][x]-=nimotu_data[26][y-1][x];
						}
					}
					for(y=1;y<=h;y++) {
						for(x=w;x>=1;x--) {
							nimotu_data[i][y][x]-=nimotu_data[i][y][x-1];
							nimotu_data[26][y][x]-=nimotu_data[26][y][x-1];
						}
					}
					/* nurinaosu */
					for(y=1;y<=h;y++) {
						for(x=1;x<=w;x++) {
							nimotu_data[i][y][x]+=nimotu_data[i][y][x-1];
							nimotu_data[26][y][x]+=nimotu_data[26][y][x-1];
							if(nimotu[y][x]-'A'==i) {
								nimotu_data[i][y][x]--;
								nimotu_data[26][y][x]++;
							}
						}
					}
					for(x=1;x<=w;x++) {
						for(y=1;y<=h;y++) {
							nimotu_data[i][y][x]+=nimotu_data[i][y-1][x];
							nimotu_data[26][y][x]+=nimotu_data[26][y-1][x];
						}
					}
				}
			}
		}
	} while(kousinn_atta);
	for(i=0;i<26;i++) {
		if(nimotu_range[i].xmin<=nimotu_range[i].xmax) {
			/* nimotu ga nokotteru */
			puts("SUSPICIOUS");
			return 1;
		}
	}
	puts("SAFE");
	return 1;
}

int main(void) {
	int n,i;
	if(scanf("%d",&n)!=1)return 1;
	for(i=0;i<n;i++) {
		if(!solve())return 1;
	}
	return 0;
}