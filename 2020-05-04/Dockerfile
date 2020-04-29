##########################
### Dockerfile Details ###
##########################
FROM archlinux/base

# setup the target version
ARG version=0.5.4

# clean up the source code
ARG clean=true

MAINTAINER yanganto@gmail.com


##########################################
### Fetch and prepare the source code  ###
##########################################
# basic
RUN mkdir /source
RUN pacman -Syu wget tar sudo unzip go --noconfirm

# darwinia
RUN mkdir -p /source/darwinia 
RUN wget https://github.com/darwinia-network/darwinia/archive/v${version}.tar.gz
RUN tar zxvf v${version}.tar.gz -C /source/darwinia --strip-components=1; rm v${version}.tar.gz
# Reference:
# Repository : https://github.com/darwinia-network/darwinia

# shadow service
RUN wget https://github.com/darwinia-network/darwinia.go/archive/master.zip
RUN unzip master.zip -d /source; rm master.zip


####################################
### Build binary from the source ###
####################################
# darwinia binary
RUN wget https://raw.githubusercontent.com/darwinia-network/darwinia-common/master/.maintain/bootstrap.sh
RUN bash bootstrap.sh && rm bootstrap.sh
RUN cd /source/darwinia; $HOME/.cargo/bin/cargo build --release
RUN mv /source/darwinia/target/release/darwinia /darwinia 
# Reference:
# build steps: https://github.com/darwinia-network/darwinia/blob/master/README.adoc#building

# shadow service binary
RUN pacman -S make gcc --noconfirm
RUN ls /source/darwinia.go-master
RUN cd /source/darwinia.go-master; make
RUN mv /source/darwinia.go-master/target/dargo /dargo
# Reference:
# build steps: https://github.com/darwinia-network/darwinia.go/blob/master/README.md


#############################
### Set up shadow service ###
#############################
RUN pacman -S nginx sed --noconfirm

# Because this is docker image, adding eth-resource is a hacky thing.
# It is simple in linux with this command
# `echo '127.0.0.1 eth-resource' >> /etc/hosts`
RUN cp /etc/hosts /tmp/hosts 
RUN echo '127.0.0.1 eth-resource' >> /tmp/hosts
RUN mkdir -p -- /lib-override && cp /usr/lib/libnss_files.so.2 /lib-override
RUN sed -i 's:/etc/hosts:/tmp/hosts:g' /lib-override/libnss_files.so.2
ENV LD_LIBRARY_PATH /lib-override

RUN wget https://raw.githubusercontent.com/yanganto/workshop/layer-one-workshop/2020-05-04/nginx.conf
RUN mv nginx.conf /etc/nginx
# Reference:
# Protocol: https://github.com/darwinia-network/darwinia-common/issues/86
# Setup: https://github.com/darwinia-network/darwinia.js/blob/master/README.md


###############################
### Services Runner Setting ###
###############################
RUN pacman -S supervisor --noconfirm
RUN wget https://raw.githubusercontent.com/yanganto/workshop/layer-one-workshop/2020-05-04/supervisor.conf


################
### Clean Up ###
################
RUN pacman -Scc --noconfirm
RUN ${clean} && rm -rf /source


WORKDIR /
ENTRYPOINT []
CMD ["supervisord", "-c", "/supervisor.conf"]