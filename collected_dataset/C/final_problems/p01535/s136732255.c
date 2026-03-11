#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_MAX 20
#define FILE_MAX 500

enum {TAG_START,CONTENT,TAG_END};

typedef struct {
	int syurui;
	const char* naiyo;
	int visible;
} dml_t;

typedef struct ds_command_t_tag {
	int visible;
	int targetNum;
	const char* target[FILE_MAX];
	struct ds_command_t_tag* next;
} ds_command_t;

typedef struct {
	int fileID;
	const char* name;
	ds_command_t* syori;
} subroutine_t;

int dml_length[N_MAX];
int dml_script[N_MAX];
dml_t dml[N_MAX][FILE_MAX];

int command_used;
ds_command_t commands[N_MAX*FILE_MAX];

int subroutine_num;
subroutine_t subroutines[N_MAX*FILE_MAX];

int N;
char fileName[N_MAX][24];
char fileContent[N_MAX][FILE_MAX+12];

int w,h;
int nowPage;
char screen[500][512];
int screenInfo[500][500];

int qsort_subroutine_comp(const void* x,const void* y) {
	const char* a=((subroutine_t*)x)->name;
	const char* b=((subroutine_t*)y)->name;
	return strcmp(a,b);
}

int find_subroutine(const char* subroutine_name) {
	int left,right,mid;
	left=0;right=subroutine_num-1;
	while(left<=right) {
		int compResult;
		mid=(left+right)/2;
		compResult=strcmp(subroutine_name,subroutines[mid].name);
		if(compResult==0)return mid;
		else if(compResult>0)left=mid+1;
		else right=mid-1;
	}
	return -1;
}

void purse_dml(int id) {
	int i;
	int startFlag=1;
	dml_length[id]=0;
	for(i=0;fileContent[id][i];i++) {
		if(fileContent[id][i]=='<') {
			int endFlag=0;
			fileContent[id][i]=0;
			i++;
			if(fileContent[id][i]=='/') {
				endFlag=1;
				i++;
			}
			dml[id][dml_length[id]].naiyo=&fileContent[id][i];
			dml[id][dml_length[id]].syurui=(endFlag?TAG_END:TAG_START);
			dml_length[id]++;
			while(fileContent[id][i]!=0 && fileContent[id][i]!='>')i++;
			fileContent[id][i]=0;
			startFlag=1;
		} else {
			if(startFlag) {
				dml[id][dml_length[id]].naiyo=&fileContent[id][i];
				dml[id][dml_length[id]].syurui=CONTENT;
				dml_length[id]++;
				startFlag=0;
			}
		}
	}

	dml_script[id]=0;
	for(i=0;i<dml_length[id];i++) {
		if(dml[id][i].syurui==TAG_START &&
				strcmp(dml[id][i].naiyo,"script")==0) {
			i++;
			if(dml[id][i].syurui==CONTENT) {
				int j;
				char target[24];
				strcpy(target,dml[id][i].naiyo);
				strcat(target,".ds");
				for(j=0;j<N;j++) {
					if(strcmp(target,fileName[j])==0) {
						dml_script[id]|=(1<<j);
					}
				}
			}
		}
	}
}

void purse_ds(int id) {
	int i,j;
	for(i=0;fileContent[id][i];i++) {
		ds_command_t** nextsyori;
		subroutines[subroutine_num].fileID=(1<<id);
		subroutines[subroutine_num].name=&fileContent[id][i];
		nextsyori=&subroutines[subroutine_num].syori;
		subroutine_num++;

		while(fileContent[id][i]!='{')i++;
		fileContent[id][i]=0;
		i++;

		while(fileContent[id][i]!='}') {
			int nexti;
			int visible;
			for(j=i;fileContent[id][j]!=';';j++);
			nexti=j+1;

			if(fileContent[id][j-2]=='u') {
				/* true */
				visible=1;
				j-=5;
			} else {
				/* false */
				visible=0;
				j-=6;
			}
			while(j>=i) {
				int targetNum;
				char* nextTarget;
				/* j is at equal mark */
				j--;
				if(fileContent[id][j]=='!') {
					visible=!visible;
					j--;
				}
				j-=7; /* skip ".visible" */
				fileContent[id][j]=0;
				while(j>=i && fileContent[id][j]!='=')j--;
				j++;
				*nextsyori=&commands[command_used++];
				(*nextsyori)->visible=visible;
				targetNum=0;
				nextTarget=strtok(&fileContent[id][j],".");
				do {
					(*nextsyori)->target[targetNum++]=nextTarget;
				} while(nextTarget=strtok(NULL,"."));
				(*nextsyori)->targetNum=targetNum;
				nextsyori=&((*nextsyori)->next);
				j--;
			}

			i=nexti;
		}
	}
}

void render(void) {
	int i,j;
	int nowStatus=0;
	int x=0,y=0;
	for(i=0;i<h;i++) {
		for(j=0;j<w;j++) {
			screen[i][j]='.';
			screenInfo[i][j]=0;
		}
		screen[i][w]=0;
	}
	for(i=0;i<dml_length[nowPage] && y<h;i++) {
		if(dml[nowPage][i].syurui==TAG_START) {
			if(strcmp(dml[nowPage][i].naiyo,"script")==0) {
				i++; /* skip content of script tag */
			} else if(strcmp(dml[nowPage][i].naiyo,"link")==0) {
				if(dml[nowPage][i+1].syurui==CONTENT) {
					char target[512];
					strcpy(target,dml[nowPage][i+1].naiyo);
					strcat(target,".dml");
					nowStatus=0;
					for(j=0;j<N;j++) {
						if(strcmp(fileName[j],target)==0) {
							nowStatus=j+1;
							break;
						}
					}
				}
			} else if(strcmp(dml[nowPage][i].naiyo,"button")==0) {
				if(dml[nowPage][i+1].syurui==CONTENT) {
					nowStatus=find_subroutine(dml[nowPage][i+1].naiyo);
					if(nowStatus<0)nowStatus=0; else {
						/* association check */
						if(dml_script[nowPage] & subroutines[nowStatus].fileID) {
							nowStatus=-nowStatus-1;
						} else {
							nowStatus=0;
						}
					}
				}
			} else if(strcmp(dml[nowPage][i].naiyo,"br")==0) {
				if(dml[nowPage][i].visible) {
					y++;
					x=0;
				}
			}
		} else if(dml[nowPage][i].syurui==TAG_END) {
			nowStatus=0;
		} else {
			if(dml[nowPage][i].visible) {
				for(j=0;dml[nowPage][i].naiyo[j] && y<h;j++) {
					screen[y][x]=dml[nowPage][i].naiyo[j];
					screenInfo[y][x]=nowStatus;
					x++;
					if(x>=w){y++;x=0;}
				}
			}
		}
	}
}

void go_link(int page_id) {
	int i;
	nowPage=page_id;
	for(i=0;i<dml_length[nowPage];i++)dml[nowPage][i].visible=1;
}

void do_script(int script_id) {
	const ds_command_t* nowsyori;
	int nowTagNum;
	const char* nowTags[FILE_MAX];
	for(nowsyori=subroutines[script_id].syori;nowsyori!=NULL;nowsyori=nowsyori->next) {
		int i;
		nowTagNum=0;
		for(i=0;i<dml_length[nowPage];i++) {
			if(dml[nowPage][i].syurui==TAG_START && strcmp(dml[nowPage][i].naiyo,"br")!=0) {
				nowTags[nowTagNum++]=dml[nowPage][i].naiyo;
			} else if(dml[nowPage][i].syurui==TAG_END) {
				if(nowTagNum>0)nowTagNum--;
			} else {
				int nowPos,j;
				for(nowPos=j=0;nowPos<nowsyori->targetNum && j<nowTagNum;j++) {
					if(strcmp(nowsyori->target[nowPos],nowTags[j])==0)nowPos++;
				}
				if(nowPos>=nowsyori->targetNum) {
					dml[nowPage][i].visible=nowsyori->visible;
				}
			}
		}
	}
}

void click(int x,int y) {
	int i;
	if(x<0 || x>=w || y<0 || y>=h)return;
	if(screenInfo[y][x]>0) {
		go_link(screenInfo[y][x]-1);
		render();
	} else if(screenInfo[y][x]<0) {
		do_script(-screenInfo[y][x]-1);
		render();
	}
}

int main(void) {
	int i;
	int M;
	scanf("%d ",&N);
	for(i=0;i<N;i++) {
		char* nl;
		fgets(fileName[i],sizeof(fileName[i]),stdin);
		if(nl=strchr(fileName[i],'\n'))*nl=0;
		fgets(fileContent[i],sizeof(fileContent[i]),stdin);
		if(nl=strchr(fileContent[i],'\n'))*nl=0;
	}
	/* do purse after reading all files! */
	for(i=0;i<N;i++) {
		char* dot;
		if((dot=strchr(fileName[i],'.')) && strcmp(dot,".ds")==0) {
			purse_ds(i);
		} else {
			purse_dml(i);
		}
	}
	qsort(subroutines,subroutine_num,sizeof(subroutine_t),qsort_subroutine_comp);
	scanf("%d",&M);
	for(i=0;i<M;i++) {
		int s;
		int j;
		char startFile[24];
		scanf("%d%d%d%s",&w,&h,&s,startFile);
		strcat(startFile,".dml");
		for(j=0;j<N;j++) {
			if(strcmp(startFile,fileName[j])==0)break;
		}
		if(j>=N)return 1;
		go_link(j);
		render();
		for(j=0;j<s;j++) {
			int x,y;
			scanf("%d%d",&x,&y);
			click(x,y);
		}
		for(j=0;j<h;j++)puts(screen[j]);
	}
	return 0;
}