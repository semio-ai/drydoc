import * as React from 'react';

import { connect } from 'react-redux';
import styled from 'styled-components';
import { State as ReduxState } from '../store';

import Page from '../state/Page';
import { RouteComponentProps, withRouter } from 'react-router';
import { Icon, NAVIGATION_BACKGROUND_COLOR } from '../style';

export interface NavBarProps {
  page: Page;

  path?: Page[];

  location?: string;

  onPageChange: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

interface NavBarState {

}

type Props = NavBarProps;
type State = NavBarState;

const Container = styled.div`
  display: flex;
  align-items: center;
  flex-direction: row;  
  width: 100%;
  height: 28px;
  font-size: 12px;
  padding: 0;
  padding-left: 8px;
  padding-right: 8px;
  background-color: ${NAVIGATION_BACKGROUND_COLOR};
`;

const ItemContainer = styled.div`
  height: 28px;
  line-height: 28px;
  padding: 0;
  background-color: ${NAVIGATION_BACKGROUND_COLOR};
  user-select: none;
  :hover {
    background-color: rgba(255, 255, 255, 0.1);
  }
  cursor: pointer;
  transition: background-color 0.2s;
`;


interface ItemProps {
  page: Page;
  terminal: boolean;

  onClick: (id: string, event: React.MouseEvent<HTMLDivElement>) => void;
}

class Item extends React.PureComponent<ItemProps> {
  private onClick_ = (event: React.MouseEvent<HTMLDivElement>) => {
    this.props.onClick(this.props.page.id, event);
  }

  render() {
    const { props } = this;
    return (
      <ItemContainer onClick={this.onClick_}>
        {props.page.name}
      </ItemContainer>
    )
  }
}

export class NavBar extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
  }

  render() {
    const { props } = this;
    if (!props.path) return null;

    return (
      <Container>
        {props.path.map((page, i) => (
          <>
            <Item onClick={props.onPageChange} page={page} terminal={i + 1 == props.path?.length} />
            {i + 1 !== props.path?.length ? <span><i style={{ marginLeft: '8px', marginRight: '8px' }} className='fa fa-xs fa-chevron-right' /></span> : undefined}
          </>
        ))}
        <div style={{ flex: '1 1' }} />
        <span><i style={{ margin: '4px' }} className='fa fa-bookmark' /></span>
        <i style={{ margin: '4px' }} className='fa fa-adjust' />
        <span style={{ marginLeft: '4px' }}>Generated by Drydoc</span>
      </Container>
    );
  }
}

export default connect((state: ReduxState, ownProps: Props) => {
  let path: Page[] = [];
  let current = ownProps.page.id;
  while (current) {
    path.push(state.page.pages[current]);
    current = state.page.byParent[current];
  }

  path.push(state.page.pages[""]);

  path = path.reverse();

  console.log('STATE', path);
  
  return {
    path
  };
}, (dispatch, ownProps) => {

})(NavBar);