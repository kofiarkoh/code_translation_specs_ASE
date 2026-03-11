#include <stdio.h>
#include <string.h>

typedef struct {
	int bunsi,bunbo;
	int oto;
} note_t;

void yakubun(note_t *note) {
	int a=note->bunsi,b=note->bunbo,r;
	if(a>0) {
		while(b>0) {
			r=a%b;
			a=b;
			b=r;
		}
		note->bunsi/=a;
		note->bunbo/=a;
	} else {
		note->bunsi=0;
		note->bunbo=1;
	}
}

int main(void) {
	int case_num,case_count;
	if(scanf("%d",&case_num)!=1)return 1;
	for(case_count=0;case_count<case_num;case_count++) {
		static note_t notes[1024*8];
		int notes_count=0;
		char input[4096];
		int N,i,j;
		int anslen;
		static int answer[1024];
		if(scanf("%d",&N)!=1)return 1;
		for(i=0;i<N;i++) {
			int len;
			if(scanf("%s",input)!=1)return 1;
			len=strlen(input)/2;
			for(j=0;input[j*2];j++) {
				char buf[4];
				int oto;
				buf[0]=input[j*2];
				buf[1]=input[j*2+1];
				buf[2]='\0';
				if(sscanf(buf,"%x",&oto)!=1)return 1;
				if(oto!=0) {
					notes[notes_count].bunsi=j;
					notes[notes_count].bunbo=len;
					notes[notes_count].oto=oto;
					yakubun(&notes[notes_count]);
					notes_count++;
				}
			}
		}
		anslen=1;
		for(i=0;i<notes_count;i++) {
			int a=anslen,b=notes[i].bunbo,r;
			while(b>0) {
				r=a%b;
				a=b;
				b=r;
			}
			anslen=anslen/a*notes[i].bunbo;
			if(anslen>1024) {
				anslen=-1;
				break;
			}
		}
		if(anslen>0) {
			for(i=0;i<anslen;i++)answer[i]=0;
			for(i=0;i<notes_count;i++) {
				int s=notes[i].bunsi;
				int b=notes[i].bunbo;
				int o=notes[i].oto;
				answer[s*(anslen/b)]|=o;
			}
			for(i=0;i<anslen;i++)printf("%02X",answer[i]);
			putchar('\n');
		} else {
			puts("Too complex.");
		}
	}
	return 0;
}