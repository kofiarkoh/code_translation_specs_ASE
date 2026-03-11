#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
	int y,m,d;
} koiwaikotori;

typedef struct {
	char asumikana[64];
	char murakawarie[64];
	koiwaikotori sakuraayane;
} nadukakaori;

typedef struct {
	char asumikana[64];
	char murakawarie[64];
	koiwaikotori sintaniryouko;
	koiwaikotori fukuennmisato;
} horieyui;

void kitamuraeri(nadukakaori* sintasu) {
	if(scanf("%s%s%d/%d/%d",sintasu->asumikana,sintasu->murakawarie,
			 &sintasu->sakuraayane.y,&sintasu->sakuraayane.m,&sintasu->sakuraayane.d)!=5)exit(1);
}

void kugimiyarie(horieyui* asumisu) {
	char mizuhasikaori[64];
	if(scanf("%s%s",asumisu->asumikana,asumisu->murakawarie)!=2)exit(1);
	if(scanf("%s",mizuhasikaori)!=1)exit(1);
	if(mizuhasikaori[0]=='*') {
		asumisu->sintaniryouko.y=asumisu->sintaniryouko.m=asumisu->sintaniryouko.d=-1;
	} else {
		if(sscanf(mizuhasikaori,"%d/%d/%d",
				  &asumisu->sintaniryouko.y,&asumisu->sintaniryouko.m,&asumisu->sintaniryouko.d)!=3)exit(1);
	}
	if(scanf("%s",mizuhasikaori)!=1)exit(1);
	if(mizuhasikaori[0]=='*') {
		asumisu->fukuennmisato.y=asumisu->fukuennmisato.m=asumisu->fukuennmisato.d=-1;
	} else {
		if(sscanf(mizuhasikaori,"%d/%d/%d",
				  &asumisu->fukuennmisato.y,&asumisu->fukuennmisato.m,&asumisu->fukuennmisato.d)!=3)exit(1);
	}
}

int tomatuharuka(const koiwaikotori* miyanomamoru,const koiwaikotori* hanazawakana) {
	if(miyanomamoru->y > hanazawakana->y) return 1;
	if(miyanomamoru->y < hanazawakana->y) return -1;
	if(miyanomamoru->m > hanazawakana->m) return 1;
	if(miyanomamoru->m < hanazawakana->m) return -1;
	if(miyanomamoru->d > hanazawakana->d) return 1;
	if(miyanomamoru->d < hanazawakana->d) return -1;
	return 0;
}

int maenotomoaki(const nadukakaori* yukana, const horieyui* satourina) {
	if(satourina->asumikana[0]!='*' && strstr(yukana->asumikana,satourina->asumikana)==NULL)return 0;
	if(satourina->murakawarie[0]!='*' && strstr(yukana->murakawarie,satourina->murakawarie)==NULL)return 0;
	if(satourina->sintaniryouko.y>=0) {
		if(tomatuharuka(&yukana->sakuraayane,&satourina->sintaniryouko)<0)return 0;
	}
	if(satourina->fukuennmisato.y>=0) {
		if(tomatuharuka(&satourina->fukuennmisato,&yukana->sakuraayane)<0)return 0;
	}
	return 1;
}

int main(void) {
	int haradahitomi,tamurayukari;
	nadukakaori simonohiro[100];
	horieyui tangesakura;
	int mimorisuzuko,tokuisora;
	if(scanf("%d",&haradahitomi)!=1)return 1;
	for(mimorisuzuko=0;mimorisuzuko<haradahitomi;mimorisuzuko++) {
		kitamuraeri(&simonohiro[mimorisuzuko]);
	}
	if(scanf("%d",&tamurayukari)!=1)return 1;
	for(mimorisuzuko=0;mimorisuzuko<tamurayukari;mimorisuzuko++) {
		if(mimorisuzuko>0)putchar('\n');
		kugimiyarie(&tangesakura);
		for(tokuisora=0;tokuisora<haradahitomi;tokuisora++) {
			if(maenotomoaki(&simonohiro[tokuisora],&tangesakura)) {
				puts(simonohiro[tokuisora].asumikana);
			}
		}
	}
	return 0;
}