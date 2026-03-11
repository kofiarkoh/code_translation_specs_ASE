#include <stdio.h>

int width,height;
char map[20][21];
int command_len;
char command[200];

int main(void) {
	int d_num,c_data;
	int i,j;
	int x,y;
	scanf("%d",&d_num);
	for(c_data=0;c_data<d_num;c_data++) {
		scanf("%d%d",&height,&width);
		for(i=0;i<height;i++)scanf("%s",map[i]);
		scanf("%d",&command_len);
		scanf("%s",command);
		x=y=-1;
		for(i=0;i<height;i++) {
			for(j=0;j<width;j++) {
				if(map[i][j]=='^' || map[i][j]=='v' ||
						map[i][j]=='<' || map[i][j]=='>') {
					y=i;x=j;
					i=height;
					break;
				}
			}
		}
		if(x<0 || y<0)return 1;
		for(i=0;i<command_len;i++) {
			switch(command[i]) {
				case 'U':
					map[y][x]='^';
					if(y>0 && map[y-1][x]=='.') {
						map[y-1][x]='^';
						map[y][x]='.';
						y--;
					}
					break;
				case 'D':
					map[y][x]='v';
					if(y<height-1 && map[y+1][x]=='.') {
						map[y+1][x]='v';
						map[y][x]='.';
						y++;
					}
					break;
				case 'L':
					map[y][x]='<';
					if(x>0 && map[y][x-1]=='.') {
						map[y][x-1]='<';
						map[y][x]='.';
						x--;
					}
					break;
				case 'R':
					map[y][x]='>';
					if(x<width-1 && map[y][x+1]=='.') {
						map[y][x+1]='>';
						map[y][x]='.';
						x++;
					}
					break;
				case 'S':
					switch(map[y][x]) {
						case '^':
							for(j=0;y-j>=0;j++) {
								if(map[y-j][x]=='*') {
									map[y-j][x]='.';
									break;
								} else if(map[y-j][x]=='#')break;
							}
							break;
						case 'v':
							for(j=0;y+j<height;j++) {
								if(map[y+j][x]=='*') {
									map[y+j][x]='.';
									break;
								} else if(map[y+j][x]=='#')break;
							}
							break;
						case '<':
							for(j=0;x-j>=0;j++) {
								if(map[y][x-j]=='*') {
									map[y][x-j]='.';
									break;
								} else if(map[y][x-j]=='#')break;
							}
							break;
						case '>':
							for(j=0;x+j<width;j++) {
								if(map[y][x+j]=='*') {
									map[y][x+j]='.';
									break;
								} else if(map[y][x+j]=='#')break;
							}
							break;
					}
					break;
			}
		}
		for(i=0;i<height;i++)puts(map[i]);
		if(c_data<d_num-1)putchar('\n');
	}
	return 0;
}