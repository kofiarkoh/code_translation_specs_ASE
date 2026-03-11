#include <stdio.h>
#include <string.h>

int size;
char hand_up[100][100];
int map[100][100];

int main(void) {
	int i,j;
	int ctarget,cpos;
	while(1) {
		scanf("%d",&size);
		if(size==0)break;
		for(i=0;i<size;i++) {
			for(j=0;j<size;j++)scanf("%d",&map[i][j]);
		}
		memset(hand_up,0,sizeof(hand_up));
		for(i=0;i<size;i++) {
			ctarget=0x7fffffff;
			cpos=-1;
			for(j=0;j<size;j++) {
				if(map[i][j]<ctarget) {
					ctarget=map[i][j];
					cpos=j;
				}
			}
			hand_up[i][cpos]|=1;
		}
		for(i=0;i<size;i++) {
			ctarget=0;
			cpos=-1;
			for(j=0;j<size;j++) {
				if(map[j][i]>ctarget) {
					ctarget=map[j][i];
					cpos=j;
				}
			}
			hand_up[cpos][i]|=2;
		}
		for(i=0;i<size;i++) {
			for(j=0;j<size;j++) {
				if(hand_up[i][j]==3)printf("%d\n",map[i][j]);
			}
		}
	}
	return 0;
}