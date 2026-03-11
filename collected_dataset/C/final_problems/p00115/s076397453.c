#include <stdio.h>

#define EPS (1e-7)

typedef struct {
	double x,y,z;
} zahyo_t;

zahyo_t read_zahyo(void) {
	zahyo_t ret;
	scanf("%lf%lf%lf",&ret.x,&ret.y,&ret.z);
	return ret;
}

zahyo_t sub_zahyo(zahyo_t dest,zahyo_t src) {
	zahyo_t ret;
	ret.x=dest.x-src.x;
	ret.y=dest.y-src.y;
	ret.z=dest.z-src.z;
	return ret;
}

double zettaiti(double a) {
	return a<0?-a:a;
}

#define WIDTH 4
#define HEIGHT 3

int gausu(double matrix[HEIGHT][WIDTH]) {
	int i,j,k;
	for(i=0;i<HEIGHT;i++) {
		/* pipotto-sentaku */
		double hiscore=zettaiti(matrix[i][i]);
		int hiindex=i;
		double atai;
		for(j=i+1;j<HEIGHT;j++) {
			if(zettaiti(matrix[j][i])>hiscore) {
				hiscore=zettaiti(matrix[j][i]);
				hiindex=j;
			}
		}
		/* koukan */
		for(j=0;j<WIDTH;j++) {
			double tmp=matrix[i][j];
			matrix[i][j]=matrix[hiindex][j];
			matrix[hiindex][j]=tmp;
		}
		if(zettaiti(matrix[i][i])<EPS)return 0;
		/* syoukyo */
		atai=matrix[i][i];
		for(j=0;j<WIDTH;j++)matrix[i][j]/=atai;
		for(j=0;j<HEIGHT;j++) {
			double keisu=matrix[j][i];
			if(j==i)continue;
			for(k=0;k<WIDTH;k++)matrix[j][k]-=keisu*matrix[i][k];
		}
	}
	return 1;
}

#undef WIDTH
#undef HEIGHT

int main(void) {
	zahyo_t me;
	zahyo_t enemy;
	zahyo_t p1,p2,p3;
	zahyo_t vec1,vec2,vec3;
	double matrix[3][4];
	int gausu_ret;
	me=read_zahyo();
	enemy=read_zahyo();
	p1=read_zahyo();
	p2=read_zahyo();
	p3=read_zahyo();
	vec1=sub_zahyo(enemy,me);
	vec2=sub_zahyo(p2,p1);
	vec3=sub_zahyo(p3,p1);
	/* me+x*vec1 == p1 + y*vec2 + z*vec3 */
	/* [vec1 -vec2 -vec3]X = p1-me */
	matrix[0][0]=vec1.x;matrix[0][1]=-vec2.x;matrix[0][2]=-vec3.x;
	matrix[1][0]=vec1.y;matrix[1][1]=-vec2.y;matrix[1][2]=-vec3.y;
	matrix[2][0]=vec1.z;matrix[2][1]=-vec2.z;matrix[2][2]=-vec3.z;
	matrix[0][3]=p1.x-me.x;
	matrix[1][3]=p1.y-me.y;
	matrix[2][3]=p1.z-me.z;
#if 0
	{
		int i,j;
		for(i=0;i<3;i++) {
			for(j=0;j<4;j++) {
				printf("\t%f",matrix[i][j]);
			}
			puts("");
		}
	}
#endif
	gausu_ret=gausu(matrix);
#if 0
	{
		int i,j;
		for(i=0;i<3;i++) {
			for(j=0;j<4;j++) {
				printf("\t%f",matrix[i][j]);
			}
			puts("");
		}
	}
#endif
	puts((!gausu_ret || /* kai nasi */
		/* x==0 -> UAZ ga baria no heimenzyo -> tuburete mieru */
		/* UAZ-teki ni baria ga nai */
		(matrix[0][3]<EPS || matrix[0][3]>1.0+EPS) ||
		/* hou ga baria ni ataranai !(y>=0 && z>=0 && y+z<=1) */
		(matrix[1][3]+EPS<0.0 || matrix[2][3]+EPS<0.0 || matrix[1][3]+matrix[2][3]>1.0+EPS)
	)?"HIT":"MISS");
	return 0;
}