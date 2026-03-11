#include <stdio.h>
#include <string.h>

int line_max;
char buffer[2000];
char bunsyou[200][2000];
int cx,cy;

void a(void) {
	cx=0;
}

void e(void) {
	cx=strlen(bunsyou[cy]);
}

void p(void) {
	if(cy>0)cy--;
	cx=0;
}

void n(void) {
	if(cy<line_max-1)cy++;
	cx=0;
}

void f(void) {
	if(cx<strlen(bunsyou[cy]))cx++;
	else if(cy<line_max-1) {
		cy++;
		cx=0;
	}
}

void b(void) {
	if(cx>0)cx--;
	else if(cy>0) {
		cy--;
		cx=strlen(bunsyou[cy]);
	}
}

void d(void) {
	int i,length;
	if(cx<strlen(bunsyou[cy])) {
		length=strlen(&bunsyou[cy][cx])-1;
		memmove(&bunsyou[cy][cx],&bunsyou[cy][cx+1],length);
		bunsyou[cy][cx+length]=0;
	} else if(cy<line_max-1) {
		strcat(bunsyou[cy],bunsyou[cy+1]);
		for(i=cy+2;i<line_max;i++) {
			strcpy(bunsyou[i-1],bunsyou[i]);
		}
		line_max--;
	}
}

void k(void) {
	int i;
	if(cx>=strlen(bunsyou[cy])) {
		if(cy<line_max-1) {
			strcat(bunsyou[cy],bunsyou[cy+1]);
			for(i=cy+2;i<line_max;i++) {
				strcpy(bunsyou[i-1],bunsyou[i]);
			}
			line_max--;
			buffer[0]='\n';
			buffer[1]=0;
		}
	} else {
		strcpy(buffer,&bunsyou[cy][cx]);
		bunsyou[cy][cx]=0;
	}
}

void y(void) {
	int i,length;
	if(buffer[0]==0)return;
	if(buffer[0]=='\n') {
		for(i=line_max;i>cy+1;i--) {
			strcpy(bunsyou[i],bunsyou[i-1]);
		}
		line_max++;
		strcpy(bunsyou[cy+1],&bunsyou[cy][cx]);
		bunsyou[cy][cx]=0;
		cy++;
		cx=0;
	} else {
		length=strlen(buffer);
		memmove(&bunsyou[cy][cx+length],&bunsyou[cy][cx],
			strlen(&bunsyou[cy][cx])+1);
		memmove(&bunsyou[cy][cx],buffer,length);
		cx+=length;
	}
}

int main(void) {
	char* cp;
	char cmd[10];
	int i;
	line_max=0;
	while(1) {
		fgets(bunsyou[line_max],sizeof(bunsyou[line_max]),stdin);
		cp=strchr(bunsyou[line_max],'\n');
		if(cp)*cp=0;
		if(strcmp(bunsyou[line_max],"END_OF_TEXT")==0) {
			bunsyou[line_max][0]=0;
			break;
		} else {
			line_max++;
		}
	}
	while(1) {
		scanf("%s",cmd);
		if(cmd[0]=='-')break;
		switch(cmd[0]) {
			case 'a':a();break;
			case 'e':e();break;
			case 'p':p();break;
			case 'n':n();break;
			case 'f':f();break;
			case 'b':b();break;
			case 'd':d();break;
			case 'k':k();break;
			case 'y':y();break;
		}
	}
	for(i=0;i<line_max;i++)puts(bunsyou[i]);
	return 0;
}