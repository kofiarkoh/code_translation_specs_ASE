#include <stdio.h>

const char* map[]={
	"         ",
	"         ",
	"    7    ",
	"   8 M   ",
	"  9 6 L  ",
	" A 1 5 K ",
	"  B 0 J  ",
	" C 2 4 I ",
	"  D 3 H  ",
	"   E G   ",
	"    F    ",
	"         ",
	"         ",
	NULL
};
const int map_d[7][2]={
	{ 2, 0},
	{ 1, 1},
	{ 1,-1},
	{ 0, 0},
	{-1, 1},
	{-1,-1},
	{-2, 0}
};

int get_func(const char *func,const int id[],int input) {
	int idx=0;
	int i;
	for(i=0;i<7;i++) {
		if((input>>id[i])&1)idx|=1<<i;
	}
	return func[idx]-'0';
}

int main(void) {
	char input[256];
	int taiou[100][7];
	const char* map_chars="0123456789ABCDEFGHIJKLM";
	int map_num;
	int i,j,k;
	int search_max;
	for(map_num=0;map_chars[map_num]!='\0';map_num++) {
		for(i=0;map[i]!=NULL;i++) {
			for(j=0;map[i][j]!='\0';j++) {
				if(map[i][j]==map_chars[map_num]) {
					for(k=0;k<7;k++) {
						char m=map[i+map_d[k][0]][j+map_d[k][1]];
						if(m<='9') {
							taiou[map_num][k]=m-'0';
						} else {
							taiou[map_num][k]=m-'A'+10;
						}
					}
				}
			}
		}
	}
	search_max=1<<map_num;
	while(scanf("%s",input)==1 && input[0]!='#') {
		int ok=1;
		int search;
		for(search=0;search<search_max;search++) {
			int new_input=0;
			for(i=0;i<7;i++) {
				new_input|=get_func(input,taiou[i],search)<<i;
			}
			if(get_func(input,taiou[0],new_input)!=(new_input&1)) {
				ok=0;
				break;
			}
		}
		puts(ok?"yes":"no");
	}
	return 0;
}