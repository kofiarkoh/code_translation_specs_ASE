#include <stdio.h>

/*
  1
2 3 5 4
  6
*/

int dice_map[200][200];
int height_map[200][200];

enum {
	ROLL_UP,
	ROLL_DOWN,
	ROLL_LEFT,
	ROLL_RIGHT,
	ROLL_NONE
};

/*
      ^ up
      |
<-left right->
      |
      v down/mae
*/

/* get first direction to roll */
int get_roll_dir(int ue,int mae) {
	const int dirtable[6][6]={
		{ROLL_NONE ,ROLL_UP   ,ROLL_RIGHT,ROLL_LEFT ,ROLL_DOWN ,ROLL_NONE},
		{ROLL_UP   ,ROLL_NONE ,ROLL_LEFT ,ROLL_RIGHT,ROLL_NONE ,ROLL_DOWN},
		{ROLL_UP   ,ROLL_RIGHT,ROLL_NONE ,ROLL_NONE ,ROLL_LEFT ,ROLL_DOWN},
		{ROLL_UP   ,ROLL_LEFT ,ROLL_NONE ,ROLL_NONE ,ROLL_RIGHT,ROLL_DOWN},
		{ROLL_UP   ,ROLL_NONE ,ROLL_RIGHT,ROLL_LEFT ,ROLL_NONE ,ROLL_DOWN},
		{ROLL_NONE ,ROLL_UP   ,ROLL_LEFT ,ROLL_RIGHT,ROLL_DOWN ,ROLL_NONE}
	};
	return dirtable[ue-1][mae-1];
}

/* get second direction to roll */
int get_roll_dir2(int ue,int mae) {
	const int dirtable[6][6]={
		{ROLL_NONE ,ROLL_LEFT ,ROLL_UP   ,ROLL_DOWN ,ROLL_RIGHT,ROLL_NONE },
		{ROLL_RIGHT,ROLL_NONE ,ROLL_UP   ,ROLL_DOWN ,ROLL_NONE ,ROLL_LEFT },
		{ROLL_LEFT ,ROLL_UP   ,ROLL_NONE ,ROLL_NONE ,ROLL_DOWN ,ROLL_RIGHT},
		{ROLL_RIGHT,ROLL_UP   ,ROLL_NONE ,ROLL_NONE ,ROLL_DOWN ,ROLL_LEFT },
		{ROLL_LEFT ,ROLL_NONE ,ROLL_UP   ,ROLL_DOWN ,ROLL_NONE ,ROLL_RIGHT},
		{ROLL_NONE ,ROLL_RIGHT,ROLL_UP   ,ROLL_DOWN ,ROLL_LEFT ,ROLL_NONE }
	};
	return dirtable[ue-1][mae-1];
}

/* there will be no third direction to roll */

void do_roll(int* ue,int* mae,int dir) {
	const int lefttable[6][6]={
		{0,4,2,5,3,0},
		{3,0,6,1,0,4},
		{5,1,0,0,6,2},
		{2,6,0,0,1,5},
		{4,0,1,6,0,3},
		{0,3,5,2,4,0}
	};
	/*
	  []
	[][][][]
	  []
	*/
	int dice[3][4];
	int temp;
	dice[1][1]=*ue;
	dice[2][1]=*mae;
	dice[1][0]=lefttable[(*ue)-1][(*mae)-1];
	dice[1][3]=7-dice[1][1];
	dice[0][1]=7-dice[2][1];
	dice[1][2]=7-dice[1][0];
	switch(dir) {
		case ROLL_UP:
			temp=dice[0][1];
			dice[0][1]=dice[1][1];
			dice[1][1]=dice[2][1];
			dice[2][1]=dice[1][3];
			dice[1][3]=temp;
			break;
		case ROLL_DOWN:
			temp=dice[0][1];
			dice[0][1]=dice[1][3];
			dice[1][3]=dice[2][1];
			dice[2][1]=dice[1][1];
			dice[1][1]=temp;
			break;
		case ROLL_LEFT:
			temp=dice[1][0];
			dice[1][0]=dice[1][1];
			dice[1][1]=dice[1][2];
			dice[1][2]=dice[1][3];
			dice[1][3]=temp;
			break;
		case ROLL_RIGHT:
			temp=dice[1][0];
			dice[1][0]=dice[1][3];
			dice[1][3]=dice[1][2];
			dice[1][2]=dice[1][1];
			dice[1][1]=temp;
			break;
	}
	*ue=dice[1][1];
	*mae=dice[2][1];
}


int main(void) {
	int n,i;
	int ue,mae;
	while(scanf("%d",&n)==1 && n>0) {
		int count[7]={0};
		int x,y;
		for(y=0;y<200;y++) {
			for(x=0;x<200;x++)dice_map[y][x]=height_map[y][x]=0;
		}
		for(i=0;i<n;i++) {
			int roll_dir;
			scanf("%d%d",&ue,&mae);
			x=y=100;
			while(1) {
				int dx=0,dy=0;
				roll_dir=get_roll_dir(ue,mae);
				switch(roll_dir) {
					case ROLL_UP:    dy=-1;break;
					case ROLL_DOWN:  dy= 1;break;
					case ROLL_LEFT:  dx=-1;break;
					case ROLL_RIGHT: dx= 1;break;
				}
				if(height_map[y][x]<=height_map[y+dy][x+dx]) {
					dx=dy=0;
					roll_dir=get_roll_dir2(ue,mae);
					switch(roll_dir) {
						case ROLL_UP:    dy=-1;break;
						case ROLL_DOWN:  dy= 1;break;
						case ROLL_LEFT:  dx=-1;break;
						case ROLL_RIGHT: dx= 1;break;
					}
					if(height_map[y][x]<=height_map[y+dy][x+dx])break;
				}
				do_roll(&ue,&mae,roll_dir);
				x+=dx;y+=dy;
			}
			height_map[y][x]++;
			dice_map[y][x]=ue;
		}
		for(y=0;y<200;y++) {
			for(x=0;x<200;x++)count[dice_map[y][x]]++;
		}
		for(x=1;x<=6;x++)printf("%d%c",count[x],x==6?'\n':' ');
	}
	return 0;
}