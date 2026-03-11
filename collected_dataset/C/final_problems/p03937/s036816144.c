#include <stdio.h>

int main(void) {
	
	int height,width;
	int flag = 0;
	scanf("%d %d",&height,&width);
	
	int nowPositionH = 0;
	int nowPositionW = 0;
	
	char mozi[height + 5][width + 5];
	for(int i = 0 ; i < height ; i ++){
		scanf("%s", mozi[i]);
	}
	int hcount = 0;
	int wcount = 0;
	
	while(1){
		if(mozi[nowPositionH][nowPositionW + 1] == '#'){
			if(mozi[nowPositionH + 1][nowPositionW] == '#'){
				flag = 1;
				break;
			}
			nowPositionW++;
		} else if(mozi[nowPositionH + 1][nowPositionW] == '#'){
			nowPositionH++;
		} else {
			flag = 1;
			break;
		}
		hcount++;
		if(nowPositionH == height -1 && nowPositionW == width -1){
			break;
		}

	}
	nowPositionH = 0;
	nowPositionW = 0;
	
	if(flag == 0){
		while(1){
			if(mozi[nowPositionH + 1][nowPositionW] == '#'){
				if(mozi[nowPositionH][nowPositionW + 1] == '#'){
					flag = 1;
					break;
				}
				nowPositionH++;
			} else if(mozi[nowPositionH][nowPositionW + 1] == '#'){
				nowPositionW++;
			} else {
				flag = 1;
				break;
			}
			wcount++;
			
			if(nowPositionH == height - 1  && nowPositionW == width - 1){
					break;
			}


		}
	}
	
	if(hcount != wcount){
		flag = 1;
	}

	
	if(flag == 0){
		printf("Possible\n");
	} else {
		printf("Impossible\n");
	}
	
	return 0;
}
