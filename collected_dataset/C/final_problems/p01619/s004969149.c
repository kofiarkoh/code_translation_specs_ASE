#include <stdio.h>

int M,N;
int Node[303];
int Anker[303];
int Pnow;
int cnt;
int rem,trgt,anker;

int chk();
void dbg_route(int okng);

int main(){

	int i;
	// 1 <= N <= 100 row
	// 1 <= M <= 2 column
	scanf("%d %d",&N,&M);

	if(M==1){
		cnt=1;
		for(i=1;i<=N;i++){
			cnt*=2;
			cnt%=1000000;
		}
		printf("%d\n",cnt);
		return 0;
	}


	for(i=0;i<303;i++) Node[i]=-1;
	Node[0]=0;
	Pnow=0;
	anker=0;
	rem=-1;

	while(Pnow>=0){
		chk();
	}
	printf("%d\n",cnt);
	return 0;
}



int chk(){

	char flg;
	int i;
	int fu,fl,fr,fd;
	int fut,flt,frt,fdt;
	fu=0;
	fl=0;
	fr=0;
	fd=0;


	//set target
	Anker[Pnow]=0;

	if(Node[Pnow]==(M+1)*(N+1)-1){
		cnt++;
		cnt%=1000000;
//dbg_route(1);

		while(Pnow>=0 && Anker[Pnow]<=1)Pnow--;

		rem=Node[Pnow+1];
		Node[Pnow+1]=-1;
		return 0;
	}


	//u-chk
	trgt=Node[Pnow]-(M+1);
	flg=0;
	if(Node[Pnow]<M+1)flg=1;	//edge chk
	for(i=0;i<=Pnow;i++) if(Node[i]==trgt) flg=1;	//been-before chk
	if(trgt<=rem) flg=1;	//back-step & search order chk
	if(!flg){
		fu=1;
		fut=trgt;
//		Pnow++;
//		Node[Pnow]=trgt;
//		rem=-1;
//		return 1;
	}
	//l-chk
	trgt=Node[Pnow]-1;
	flg=0;
	if(Node[Pnow]%(M+1)==0)flg=1;	//edge chk
	for(i=0;i<=Pnow;i++) if(Node[i]==trgt) flg=1;	//been-before chk
	if(trgt<=rem) flg=1;	//back-step & search order chk
	if(!flg){
		fl=1;
		flt=trgt;
//		Pnow++;
//		Node[Pnow]=trgt;
//		rem=-1;
//		return 2;
	}
	//r-chk
	trgt=Node[Pnow]+1;
	flg=0;
	if(Node[Pnow]%(M+1)==M)flg=1;	//edge chk
	for(i=0;i<=Pnow;i++) if(Node[i]==trgt) flg=1;	//been-before chk
	if(trgt<=rem) flg=1;	//back-step & search order chk
	if(!flg){
		fr=1;
		frt=trgt;
//		Pnow++;
//		Node[Pnow]=trgt;
//		rem=-1;
//		return 3;
	}
	//d-chk
	trgt=Node[Pnow]+(M+1);
	flg=0;
	if(Node[Pnow]>(M+1)*N-1)flg=1;	//edge chk
	for(i=0;i<=Pnow;i++) if(Node[i]==trgt) flg=1;	//been-before chk
	if(trgt<=rem) flg=1;	//back-step & search order chk
	if(!flg){
		fd=1;
		fdt=trgt;
//		Pnow++;
//		Node[Pnow]=trgt;
//		rem=-1;
//		return 4;
	}

	Anker[Pnow]=fu+fl+fr+fd;
	if(Anker[Pnow]){
		Pnow++;
		rem=-1;
		if(fu){	Node[Pnow]=fut;return 1;}
		if(fl){	Node[Pnow]=flt;return 2;}
		if(fr){	Node[Pnow]=frt;return 3;}
		if(fd){	Node[Pnow]=fdt;return 4;}
	}

	if(Anker[Pnow]==0){
//dbg_route(0);
	while(Pnow>=0 && Anker[Pnow]<=1)Pnow--;

	rem=Node[Pnow+1];
	Node[Pnow+1]=-1;
	return 5;
	}
}

void dbg_route(int okng){
	int i;
	for(i=0;i<=Pnow;i++){
		printf(i==Pnow?"%3d":"%3d-",Node[i]);
	}
	printf("\t%s\n",okng?"OK":"NG");
}