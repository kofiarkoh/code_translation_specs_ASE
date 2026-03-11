#include <stdio.h>
#include <string.h>

int width,height;
char lmap[50][51];
char rmap[50][51];

int isValid(int x1,int y1,int x2,int y2) {
	if(x1<0 || x1>=width || y1<0 || y1>=height)return 0;
	if(x2<0 || x2>=width || y2<0 || y2>=height)return 0;
	if(lmap[y1][x1]=='#' || rmap[y2][x2]=='#')return 0;
	if(lmap[y1][x1]=='%' && rmap[y2][x2]=='%')return 1;
	if(lmap[y1][x1]=='%' || rmap[y2][x2]=='%')return 0;
	return 1;
}

int stack_num;
char stack[50*50*50*50][4];
char visited[50][50][50][50];

void stack_push(int x1,int y1,int x2,int y2) {
	if(!visited[y1][x1][y2][x2]) {
		stack[stack_num][0]=x1;
		stack[stack_num][1]=y1;
		stack[stack_num][2]=x2;
		stack[stack_num][3]=y2;
		stack_num++;
		visited[y1][x1][y2][x2]=1;
	}
}

int tansaku(int sx1,int sy1,int sx2,int sy2,int dx1,int dy1,int dx2,int dy2) {
	memset(visited,0,sizeof(visited));
	visited[sy1][sx1][sy2][sx2]=1;
	stack[0][0]=sx1;
	stack[0][1]=sy1;
	stack[0][2]=sx2;
	stack[0][3]=sy2;
	stack_num=1;
	while(stack_num>0 && !visited[dy1][dx1][dy2][dx2]) {
		int x1,x2,y1,y2;
		int xx1,xx2,yy1,yy2;
		stack_num--;
		x1=stack[stack_num][0];y1=stack[stack_num][1];
		x2=stack[stack_num][2];y2=stack[stack_num][3];

		yy1=y1;yy2=y2;
		if(y1>0 && lmap[y1-1][x1]!='#')yy1--;
		if(y2>0 && rmap[y2-1][x2]!='#')yy2--;
		if(isValid(x1,yy1,x2,yy2) && (y1!=yy1 || y2!=yy2)) {
			stack_push(x1,yy1,x2,yy2);
		}
		yy1=y1;yy2=y2;
		if(y1+1<height && lmap[y1+1][x1]!='#')yy1++;
		if(y2+1<height && rmap[y2+1][x2]!='#')yy2++;
		if(isValid(x1,yy1,x2,yy2) && (y1!=yy1 || y2!=yy2)) {
			stack_push(x1,yy1,x2,yy2);
		}
		xx1=x1;xx2=x2;
		if(x1>0 && lmap[y1][x1-1]!='#')xx1--;
		if(x2+1<width && rmap[y2][x2+1]!='#')xx2++;
		if(isValid(xx1,y1,xx2,y2) && (x1!=xx1 || x2!=xx2)) {
			stack_push(xx1,y1,xx2,y2);
		}
		xx1=x1;xx2=x2;
		if(x1+1<width && lmap[y1][x1+1]!='#')xx1++;
		if(x2>0 && rmap[y2][x2-1]!='#')xx2--;
		if(isValid(xx1,y1,xx2,y2) && (x1!=xx1 || x2!=xx2)) {
			stack_push(xx1,y1,xx2,y2);
		}
	}
	return visited[dy1][dx1][dy2][dx2];
}

int tansaku_wrapper(int s[4],int g[4]) {
	return tansaku(s[0],s[1],s[2],s[3],g[0],g[1],g[2],g[3]);
}

int main(void) {
	int x,y;
	int start[4]={-1,-1,-1,-1};
	int goal[4]={-1,-1,-1,-1};
	while(scanf("%d%d",&width,&height)==2 && (width|height)!=0) {
		for(y=0;y<height;y++)scanf("%s%s",lmap[y],rmap[y]);
		for(y=0;y<height;y++) {
			for(x=0;x<width;x++) {
				if(lmap[y][x]=='L') {
					start[0]=x;start[1]=y;
				}
				if(rmap[y][x]=='R') {
					start[2]=x;start[3]=y;
				}
				if(lmap[y][x]=='%') {
					goal[0]=x;goal[1]=y;
				}
				if(rmap[y][x]=='%') {
					goal[2]=x;goal[3]=y;
				}
			}
		}
		if(start[0]<0 || goal[0]<0 || start[2]<0 || goal[2]<0)return 1;
		puts(tansaku_wrapper(start,goal)?"Yes":"No");
	}
	return 0;
}