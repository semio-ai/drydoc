import * as React from 'react';
import styled from 'styled-components';

import PageModel from '../../state/Page';
import { Service } from '../ros/Service';
import { MONOSPACE_FONT_FAMILY, Title } from '../ros/style';

export interface RosServiceProps {
  page: PageModel.Resolved;

  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

const Container = styled.div`
  padding: 20px;
  width: 100%;
  overflow-y: auto;
  overflow-x: hidden;
`;

type Props = RosServiceProps;

export class RosService extends React.PureComponent<Props> {
  constructor(props: Props) {
    super(props);

    this.state = {};
  }
  
  render() {
    const { props } = this;

    const service = JSON.parse(props.page.content);

    return (
      <Container>
        <Title style={{ fontFamily: MONOSPACE_FONT_FAMILY }}>{`${service.package}/${service.name}`}</Title>
        <Service service={service} />
      </Container>
    );
  }
}