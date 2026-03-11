#include<stdio.h>
int W,H,map[20][20]={{0}};
char tile[20][21];
void search(int h,int w){
	if(tile[h][w]=='#'||map[h][w]==1||h<0||h>=H||w<0||w>=W)return;
	map[h][w]=1;
	search(h,w+1);
	search(h,w-1);
	search(h-1,w);
	search(h+1,w);
}
void init(void){int i,j;
		for(i=0;i<20;i++){for(j=0;j<20;j++)map[i][j]=0;}
}
int main(void){
	int i,j,memH,memW,p;
	while(1){
	p=0;
	init();
	scanf("%d%d",&W,&H);
	if(W==0&&H==0)break;
	for(i=0;i<H;i++){scanf("%s",tile[i]);}
	for(i=0;i<H;i++){for(j=0;j<W;j++){if(tile[i][j]=='@'){memH=i;memW=j;}}}
	search(memH,memW);
		for(i=0;i<H;i++){for(j=0;j<W;j++){if(map[i][j]==1)p++;}}
	printf("%d\n",p);
	}
	return 0;
}