#define _CRT_SECURE_NO_DEPRECATE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LEN 64

#define MAX_NODE 32
#define MAX_MAIL 1000
#define MAX_LABEL 50+1

#define MAX_TIME (1<<31)
#define MAX_DIST 10000
#define MAX_DISTSUM 320000

#define TRUE 1
#define FALSE 0

typedef enum {
	NONE, /* Í¯çêÄ¢È¢ */
	POST, /* XÖÇÉ é */
	DELI, /* zB */
	DONE  /* zB®¹ */
} MailState;

typedef enum {
	HERE, /* zBõª¢é */
	GO,   /* zBõªÚInÉü©ÁÄ¢é */
	BACK  /* zBõªÚIn©çAÁÄ«Ä¢é */
} NodeState;

typedef struct _Mail{
	MailState state;    /* [ÌóÔ */
	unsigned long timer;          /* XÖÇÉÍ­Ô */
	struct _Mail *next; /* ÉÍ¢½[ */
	int src;            /* a©èê */
	int dst;            /* ÅIÍ¯æ */
	char label[MAX_LABEL]; /* x */
} Mail;

typedef struct {
	NodeState state;    /* zBõÌóÔ */
	unsigned long timer;          /* zBõªÚ®µIíéÔ */
	int dst;            /* zBõÌÚIn */
	short dist[MAX_NODE]; /* ]oHÌ£ */
	char distf[MAX_NODE]; /* ÅZ£vZÏÝtO*/
	int distnext[MAX_NODE]; /* ÅZ£ÌÆ«É^Ôm[h*/
	Mail *post_queue;        /* Í¢Ä¢é[ */
	Mail *deli_queue;        /* zBÌ[ */
} Node;

/* nÂÌXÖÇim[hjÌú» */
void init_nodes(int n);

/* Í¯çêÄ¢È¢[Ì */
int check_mail(int l);
/* XÖÇÔÌÅZ£vZ */
int calc_dist(int a, int b);
/* èzBJn */
void take_mail(int n);
/* [Ì */
void post_mail(Mail *m);
/* zB®¹ */
void arrive_mail(int n);
/* zBõÌAÒõ */
void go_back(int n);
/* zB®¹[Ì\¦ */
void print_arrive(void);


void del_mail(Mail **q,Mail *mail);
void add_mail(Mail **q,Mail *mail);

int compare_mail(const void* a, const void* b);

Node node[MAX_NODE]; /* XÖÇXg */
int node_num; /* XÖÇgp */
Mail mail[MAX_MAIL]; /* [Xg */
Mail mmail; /* ¢[Xg(æªÍ_~[) */
int mail_num; /* [gp */
Mail *arrive[MAX_MAIL];/* zB®¹[Xg */
int arrive_cnt;/* zB®¹[ */

unsigned long time = 0; /* V~[VÔ */

int main(){
	char buf[MAX_LEN];
	int i;
	int n,m;
	int a,b,d;
	int l;
	Mail *lmail,*nmail;
	unsigned long min; /* ñ­Î·é^C}l */
	
	while(fgets(buf,MAX_LEN,stdin)!=NULL){
		if(sscanf(buf,"%d %d",&n,&m)<2){
			fprintf(stderr,"error:invalid input\n");
		}else{
			/* 0 0 ¾Á½çI¹ */
			if(n<2||m<1)break;
			/* KvÈª¾¯ú» */
			init_nodes(n);
			/* ]oHÇÝæè */
			for(i=0;i<m;i++){
				if(fgets(buf,MAX_LEN,stdin)==NULL)return 1;
				if(sscanf(buf,"%d %d %d",&a,&b,&d)<3)return 1;
				/* 0IWÉ·é½ß-1 */
				node[a-1].dist[b-1] = d;
				node[b-1].dist[a-1] = d;
				node[a-1].distnext[b-1] = b-1;
				node[b-1].distnext[a-1] = a-1;
			}
			/* [ÇÝæè */
			if(fgets(buf,MAX_LEN,stdin)==NULL)return 1;
			if(sscanf(buf,"%d",&l)<1)return 1;
			/* [îñÇÝæè */
			min = MAX_TIME;/* ÅÉ­Î·é^C}Í[ */
			mmail.next = NULL;
			for(i=0;i<l;i++){
				if(fgets(buf,MAX_LEN,stdin)==NULL)return 1;
				if(sscanf(buf,"%d %d %d %[a-zA-Z0-9-_]",
					&(mail[i].src),&(mail[i].dst),&(mail[i].timer),mail[i].label)<4)return 1;
				/* 0IWÉ·é½ß-1 */
				mail[i].src--;
				mail[i].dst--;
				mail[i].state = NONE;
				mail[i].next = mmail.next;
				mmail.next = &mail[i];
				if(min>mail[i].timer)min = mail[i].timer;
			}
			node_num = n;
			mail_num = l;

			/* V~[VJn */
			time = min;
			arrive_cnt = 0;
			while(check_mail(l)!=0){/* S[ªzBI¹·éÜÅ */
				min = MAX_TIME;
				/* èªÍ­ */
				lmail = &mmail;
				nmail = lmail->next;
				while(nmail!=NULL){
					if(nmail->timer <= time){
						lmail->next = nmail->next;
						post_mail(nmail);
					}else{
						if(min>nmail->timer)min=nmail->timer;
						lmail = lmail->next;
					}
					nmail = lmail->next;
				}
				for(i=0;i<n;i++){
					/* zBõª·é */
					if(node[i].timer == time){
						if(node[i].state == GO){
							arrive_mail(i);
							go_back(i);
						}else if(node[i].state == BACK){
							node[i].timer = 0;
							node[i].state = HERE;
						}
					}
					/* zBõªo­·é */
					if(node[i].state == HERE){
						take_mail(i);
					}
					if(node[i].timer && min>node[i].timer)min=node[i].timer;
				}
				print_arrive();
				time=min;
			}
			printf("\n");
		}
	}
	return 0;
}

void init_nodes(int n){
	int i,j;
	for(i=0;i<n;i++){
		node[i].state = HERE;
		node[i].timer = 0;
		for(j=0;j<n;j++){
			node[i].dist[j] = MAX_DIST;
			node[i].distf[j] = FALSE;
			node[i].distnext[j] = -1;
		}
		node[i].dist[i] = 0;
		node[i].distf[i] = TRUE;
		node[i].distnext[i] = i;
		node[i].dst = -1;
		node[i].post_queue = NULL;
		node[i].deli_queue = NULL;
	}
}

int check_mail(int l){
	int i,cnt;
	cnt=0;
	for(i=0;i<l;i++){
		if(mail[i].state != DONE)cnt++;
	}
	return cnt;
}

void post_mail(Mail *m){
	add_mail(&(node[m->src].post_queue),m);
	m->state = POST;
}

void take_mail(int n){
	Mail *que,**q,*tmp;

	/* zB·é[ª é©H */
	if(node[n].post_queue!=NULL){

		/* ÅÌ[ðzBXgÖÚ® */
		que = node[n].post_queue;
		del_mail(&(node[n].post_queue),que);
		add_mail(&(node[n].deli_queue),que);

		que->state = DELI;
		node[n].state = GO;
		node[n].timer = time+calc_dist(n,que->dst);/* »Ý{ÚWnÜÅÌ£ */
		node[n].dst = node[n].distnext[que->dst];

		/* ¯¶Í¯æÌ[ðzBXgÖÚ® */
		q = &(node[n].post_queue);
		while(*q != NULL){
			if((*q)->dst == que->dst){
				tmp = *q;
				tmp->state = DELI;
				del_mail(q,tmp);
				add_mail(&(node[n].deli_queue),tmp);
			}else{
				q = &((*q)->next);
			}
		}
	}
}

void arrive_mail(int n){
	Mail *tmp;

	while(node[n].deli_queue != NULL){
		tmp = node[n].deli_queue;
		del_mail(&(node[n].deli_queue),tmp);
		if(tmp->dst==node[n].dst){
			/* ÅIÍ¯æÉ */
			arrive[arrive_cnt] = tmp;
			arrive_cnt++;
			tmp->state = DONE;
		}else{
			/* ³çÉ] */
			add_mail(&(node[node[n].distnext[tmp->dst]].post_queue),tmp);
			tmp->state = POST;
		}
	}

}

void go_back(int n){
	node[n].state = BACK;
	node[n].timer = time + node[n].dist[node[n].dst];
}

void del_mail(Mail **q,Mail *mail){
	while((*q)!=NULL && (*q)!=mail)q = &((*q)->next);
	if((*q)==mail){
		*q = mail->next;
		mail->next = NULL;
	}
}

void add_mail(Mail **q,Mail *mail){
	while((*q)!=NULL)q = &((*q)->next);
	mail->next = NULL;
	*q = mail;
}



int calc_dist(int a, int b){
	static int past[MAX_NODE];/* ßÉÊÁ½m[h */
	static int best[MAX_NODE];/* ÅZÌm[h */
	static int past_n = 0; /* ßÉÊÁ½m[h */
	static int dist = 0;  /* ßÉÊÁÄ«½£ */
	static int bestd = MAX_DISTSUM;  /* ÅZÌ£ */
	int i,j;

	if(past_n==0) bestd = MAX_DISTSUM;/* TõJnú» */

	past[past_n++] = a;

	/* ½Çè¢½Æ« */
	if(a==b){
		/* ¡ñÌoHªÅZÅ êÎÅZoHðXV */
		if(bestd>dist){
			bestd = dist;
			for(i=0;i<past_n;i++)best[i]=past[i];
		}
	}else{
		/* êxvZ³êÄ¢é */
		if(node[a].distf[b]==TRUE){
			i = node[a].distnext[b];
			for(j=0;j<past_n;j++)if(past[j]==i)return -1;
			dist += node[a].dist[i];
			calc_dist(i,b);
			dist -= node[a].dist[i];
		}else{
			for(i=0;i<node_num;i++){
				for(j=0;j<past_n;j++)if(past[j]==i)break;
				if(j!=past_n)continue;
				if(MAX_DIST>node[a].dist[i]){
					dist += node[a].dist[i];
					calc_dist(i,b);
					dist -= node[a].dist[i];
				}
			}
		}
	}
	past_n--;

	/* TõI¹Ê½f */
	if(past_n==0){
		for(i=0;best[i]!=b;i++){
			node[best[i]].distnext[b] = best[i+1];
			node[best[i+1]].distnext[a] = best[i];
			node[best[i]].distf[b] = TRUE;
			node[best[i+1]].distf[a] = TRUE;
		}
	}
	return  node[a].dist[node[a].distnext[b]];
}



#if 0
	do{
		min = 10000;
		min_i = 10000;
		for(i=0;i<node_num;i++){
			if(a!=i && node[a].distf[i] == FALSE){
				if(min>node[a].dist[i]){
					min = node[a].dist[i];
					min_i = i;
				}
			}
		}
		node[a].distf[min_i] = TRUE;
		node[min_i].distf[a] = TRUE;
		for(i=0;i<node_num;i++){
			if(min_i!=i && node[min_i].dist[i] != 10000){
				if(node[a].dist[i]>node[a].dist[min_i]+node[min_i].dist[i]){
					node[a].dist[i] = node[a].dist[min_i]+node[min_i].dist[i];
					node[a].distnext[i] = node[a].distnext[min_i];
				}
			}
		}
	}while(min_i!=b);


	//min = node[a].dist[b];
	//for(i=0;i<node_num;i++){
	//	if(a!=i && node[a].dist[i] != 10000){
	//		tmp = node[a].dist[i]+calc_dist(i,b);
	//		if(min>tmp)min = tmp;
	//	}
	//}
	node[a].distf[b] = TRUE;
	node[b].distf[a] = TRUE;
	node[a].dist[b] = min;
	node[b].dist[a] = min;
	return node[a].dist[node[a].distnext[b]];
}
#endif

void print_arrive(void){
	int i;
	if(arrive_cnt > 0){
		qsort(arrive,arrive_cnt,sizeof(Mail*),compare_mail);
		for(i=0;i<arrive_cnt;i++){
			printf("%s %d\n",arrive[i]->label,time);
		}
	}
	arrive_cnt = 0;
}
	

int compare_mail(const void* a, const void* b){
	return strcmp((*(Mail**)a)->label,(*(Mail**)b)->label);
}