#include <stdio.h>

const char dice_number[9][7][8]={
	{
		"#######",
		"#.....#",
		"#...|.#",
		"#.....#",
		"#...|.#",
		"#..-..#",
		"#######"
	},{
		"#######",
		"#..-..#",
		"#...|.#",
		"#..-..#",
		"#.|...#",
		"#..-..#",
		"#######"
	},{
		"#######",
		"#..-..#",
		"#...|.#",
		"#..-..#",
		"#...|.#",
		"#..-..#",
		"#######"
	},{
		"#######",
		"#.....#",
		"#.|.|.#",
		"#..-..#",
		"#...|.#",
		"#.....#",
		"#######"
	},{
		"#######",
		"#..-..#",
		"#.|...#",
		"#..-..#",
		"#...|.#",
		"#..-..#",
		"#######"
	},{
		"#######",
		"#..-..#",
		"#.|...#",
		"#..-..#",
		"#.|.|.#",
		"#..-..#",
		"#######"
	},{
		"#######",
		"#..-..#",
		"#...|.#",
		"#.....#",
		"#...|.#",
		"#.....#",
		"#######"
	},{
		"#######",
		"#..-..#",
		"#.|.|.#",
		"#..-..#",
		"#.|.|.#",
		"#..-..#",
		"#######"
	},{
		"#######",
		"#..-..#",
		"#.|.|.#",
		"#..-..#",
		"#...|.#",
		"#..-..#",
		"#######"
	}
};

char dice_map[21][60];

void rotate_90(int x,int y) {
	char buf[7][7];
	int xx,yy;
	for(yy=0;yy<7;yy++) {
		for(xx=0;xx<7;xx++) {
			buf[6-xx][yy]=dice_map[yy+y][xx+x];
		}
	}
	for(yy=0;yy<7;yy++) {
		for(xx=0;xx<7;xx++) {
			dice_map[yy+y][xx+x]=buf[yy][xx];
			if(dice_map[yy+y][xx+x]=='-')dice_map[yy+y][xx+x]='|';
			else if(dice_map[yy+y][xx+x]=='|')dice_map[yy+y][xx+x]='-';
		}
	}
}

void rotate_270(int x,int y) {
	char buf[7][7];
	int xx,yy;
	for(yy=0;yy<7;yy++) {
		for(xx=0;xx<7;xx++) {
			buf[xx][6-yy]=dice_map[yy+y][xx+x];
		}
	}
	for(yy=0;yy<7;yy++) {
		for(xx=0;xx<7;xx++) {
			dice_map[yy+y][xx+x]=buf[yy][xx];
			if(dice_map[yy+y][xx+x]=='-')dice_map[yy+y][xx+x]='|';
			else if(dice_map[yy+y][xx+x]=='|')dice_map[yy+y][xx+x]='-';
		}
	}
}

void flip_lr(int x,int y) {
	char temp;
	int xx,yy;
	for(yy=0;yy<7;yy++) {
		for(xx=0;xx<3;xx++) {
			temp=dice_map[yy+y][xx+x];
			dice_map[yy+y][xx+x]=dice_map[yy+y][6-xx+x];
			dice_map[yy+y][6-xx+x]=temp;
		}
	}
}

void flip_ud(int x,int y) {
	char temp;
	int xx,yy;
	for(xx=0;xx<7;xx++) {
		for(yy=0;yy<3;yy++) {
			temp=dice_map[yy+y][xx+x];
			dice_map[yy+y][xx+x]=dice_map[6-yy+y][xx+x];
			dice_map[6-yy+y][xx+x]=temp;
		}
	}
}

int read_number(int x,int y) {
	int i;
	int xx,yy;
	for(i=0;i<9;i++) {
		for(yy=0;yy<7;yy++) {
			for(xx=0;xx<7;xx++) {
				if(dice_map[yy+y][xx+x]!=dice_number[i][yy][xx]) {
					yy=10000;
					break;
				}
			}
		}
		if(xx>=7 && yy>=7)return i+1;
	}
	return 0;
}

int main(void) {
	int i,j;
	int player_num[6];
	int ozisan_num[6];
	int high,low;
	while(1) {
		scanf("%s",dice_map[0]);
		if(dice_map[0][0]=='0' && dice_map[0][1]==0)break;
		for(i=1;i<21;i++)scanf("%s",dice_map[i]);

		flip_lr(7,0);
		rotate_270(0,7);flip_lr(0,7);
		flip_lr(7,7);
		rotate_90(14,7);flip_lr(14,7);
		flip_lr(21,7);
		flip_lr(7,14);flip_ud(7,14);

		flip_lr(7+29,0);
		rotate_270(0+29,7);flip_lr(0+29,7);
		flip_lr(7+29,7);
		rotate_90(14+29,7);flip_lr(14+29,7);
		flip_lr(21+29,7);
		flip_lr(7+29,14);flip_ud(7+29,14);

		player_num[0]=read_number(7,0);
		player_num[1]=read_number(0,7);
		player_num[2]=read_number(7,7);
		player_num[3]=read_number(14,7);
		player_num[4]=read_number(21,7);
		player_num[5]=read_number(7,14);

		ozisan_num[0]=read_number(7+29,0);
		ozisan_num[1]=read_number(0+29,7);
		ozisan_num[2]=read_number(7+29,7);
		ozisan_num[3]=read_number(14+29,7);
		ozisan_num[4]=read_number(21+29,7);
		ozisan_num[5]=read_number(7+29,14);

		high=low=0;
		for(i=0;i<6;i++) {
			for(j=0;j<6;j++) {
				if(player_num[i]>ozisan_num[j])high++;
				else if(player_num[i]<ozisan_num[j])low++;
			}
		}
		puts(high>=low?"HIGH":"LOW");
	}
	return 0;
}